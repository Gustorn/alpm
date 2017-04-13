//! A library to access the functionality of libalpm (the library used by pacman).
//!
//! # Getting started
//! The main struct here is `Alpm`. It is responsible for wrapping an alpm database and filesystem,
//! and providing functionality for thtat alpm instance. For example...
//!
//! ```ignore
//! use libalpm::Alpm;
//! use libalpm::util;
//!
//! // get the architecture (e.g. x86_64).
//! let arch = util::uname().machine().to_owned();
//!
//! let alpm = Alpm::new("/", "/var/lib/pacman"); // default locations on arch linux
//! alpm
//! ```

#![feature(untagged_unions)]
#![feature(pub_restricted)]

extern crate alpm_sys;
extern crate url;
extern crate libc;
extern crate printf;
extern crate chrono;
#[macro_use] extern crate lazy_static;

mod error;
mod event;
mod package;
mod db;
mod pgp;
mod log;
mod callbacks;
mod options;
mod types;
pub mod util;

use std::ffi::{CString, CStr};
use std::ops::Drop;
use std::path::{PathBuf};
use std::sync::Mutex;
use std::borrow::Borrow;
use std::mem;
use std::ptr;
use std::any::Any;
use std::marker::PhantomData;

use alpm_sys::*;
use libc::{c_char, c_void};

pub use options::{Options, RepoOptions};
pub use error::{Error, AlpmResult};
pub use log::{LogLevel, LogLevels};
pub use event::Event;
pub use package::{Package, PackageRef, Group, PackageVersion, PackageFrom, Reason, Validation,
    ValidationMethod, Dependency, FileList, File, Backup, VersionConstraintType};
pub use db::Db;
pub use pgp::SigLevel;
pub use types::{Caps, DownloadResult};
use callbacks::{alpm_cb_log, alpm_cb_download, alpm_cb_totaldl, alpm_cb_fetch, alpm_cb_event};

// callbacks
lazy_static! {
    static ref LOG_CB: Mutex<Option<Box<FnMut(LogLevels, String) + Send>>> = Default::default();
    static ref DOWNLOAD_CB: Mutex<Option<Box<FnMut(&str, u64, u64) + Send>>> = Default::default();
    static ref FETCH_CB: Mutex<Option<Box<FnMut(&str, &str, bool) -> DownloadResult + Send>>> = Default::default();
    static ref DLTOTAL_CB: Mutex<Option<Box<FnMut(u64) + Send>>> = Default::default();
    static ref EVENT_CB: Mutex<Option<Box<FnMut(Event) + Send>>> = Default::default();
    //static ref QUESTION_CB: Mutex<Option<Box<FnMut(LogLevels, String) + Send>>> = Default::default();
    //static ref PROGRESS_CB: Mutex<Option<Box<FnMut(LogLevels, String) + Send>>> = Default::default();
}

/// A handle on an alpm instance
///
/// Note that I have NOT checked whether the interface is threadsafe, so it's best to use only one
/// instance of Alpm at present (doing your own synchronization if you want to share between
/// threads). Also, callbacks must be stored in global state, so if they are changed for one they
/// will be changed for all.
#[derive(Debug)]
pub struct Alpm {
    handle: *const Struct_alpm_handle,
}

impl Alpm {
    /// Get a handle on the alpm instance defined by the given root/db_path
    pub fn new(root: &str, db_path: &str) -> AlpmResult<Alpm> {
        // Requires alloc, but str is more standard
        let root = CString::new(root)?;
        let db_path = CString::new(db_path)?;
        unsafe {
            let mut err: alpm_errno_t = 0;
            let handle = alpm_initialize(root.as_ptr(), db_path.as_ptr(), &mut err);
            if err != 0 {
                Err(Error::from(err))
            } else {
                let alpm = Alpm {
                    handle: handle
                };
                Ok(alpm)
            }
        }
    }

    /// Creates an alpm instance with the given options.
    ///
    /// TODO will only be implemented after the rest of the library is finished.
    pub fn with_options(options: &Options) -> AlpmResult<Alpm> {
        unimplemented!()
    }

    /// Gets the current (last) error status. Most functions use this internally to get the
    /// error type to return, so there isn't much need to use this externally.
    pub fn error(&self) -> Option<Error> {
        let code = unsafe { alpm_errno(self.handle) };
        if code == 0 {
            None
        } else {
            Some(code.into())
        }
    }

    /// Logs a message using alpm's built in logging functionality.
    ///
    /// TODO test if all prefixes are allowed, or just DEBUG etc., & test generally
    pub fn log_action<T, U>(&mut self, prefix: &str, msg: &str) -> AlpmResult<()> {
        let prefix = CString::new(prefix)?;
        let msg = CString::new(msg.replace("%", "%%"))?;
        let res = unsafe {alpm_logaction(self.handle, prefix.as_ptr(), msg.as_ptr()) };
        if res == 0 {
            Ok(())
        } else {
            Err(self.error().unwrap_or(Error::__Unknown))
        }
    }

    /// Fetch a remote pkg from the given URL and return its path.
    pub fn fetch_pkgurl(&mut self, url: &str) -> AlpmResult<PathBuf> {
        unsafe {
            let url = CString::new(url)?;
            let path = alpm_fetch_pkgurl(self.handle, url.as_ptr());
            if path.is_null() {
                Err(Error::__Unknown)
            } else {
                // copy path into rust alloc'd data struct
                let path_rust = PathBuf::from(CStr::from_ptr(path).to_str()?);
                libc::free(path as *mut c_void);
                Ok(path_rust)
            }
        }
    }

    /// Set the callback called when a log message is received.
    pub fn log_function<F>(&mut self, func: F)
        where F: FnMut(LogLevels, String) + Send + 'static
    {
        let mut cb = LOG_CB.lock().unwrap();
        (*cb) = Some(Box::new(func));
        unsafe { alpm_option_set_logcb(self.handle, Some(alpm_cb_log)); }
    }

    /// Clears the log callback.
    pub fn clear_log_function(&mut self) {
        let mut cb = LOG_CB.lock().unwrap();
        (*cb) = None;
        unsafe { alpm_option_set_logcb(self.handle, None); }
    }

    /// Set the callback called to report progress on downloading a file.
    pub fn file_download_progress_function<F>(&mut self, func: F)
        where F: FnMut(&str, u64, u64) + Send + 'static
    {
        let mut cb = DOWNLOAD_CB.lock().unwrap();
        (*cb) = Some(Box::new(func));
        unsafe { alpm_option_set_dlcb(self.handle, Some(alpm_cb_download)); }
    }

    /// Clears the file download progress callback.
    pub fn clear_file_download_progress_function(&mut self) {
        let mut cb = DOWNLOAD_CB.lock().unwrap();
        (*cb) = None;
        unsafe { alpm_option_set_dlcb(self.handle, None); }
    }

    /// Set the callback called to report progress on total download
    pub fn total_download_progress_function<F>(&mut self, func: F)
        where F: FnMut(u64) + Send + 'static
    {
        let mut cb = DLTOTAL_CB.lock().unwrap();
        (*cb) = Some(Box::new(func));
        unsafe { alpm_option_set_totaldlcb(self.handle, Some(alpm_cb_totaldl)); }
    }

    /// Clears the total download progress callback.
    pub fn clear_total_download_progress_function(&mut self) {
        let mut cb = DLTOTAL_CB.lock().unwrap();
        (*cb) = None;
        unsafe { alpm_option_set_totaldlcb(self.handle, None); }
    }

    /// Set the callback called to download a file.
    ///
    /// Providing this function is optional and it is recommended that you don't set it (and use
    /// the built-in fetch fn). This could be useful e.g. if you are behind a complicated proxy or
    /// want to use something other than http to fetch.
    ///
    /// # Safety
    /// Note that if you supply this function, you promise that if you return DownloadResult::Ok,
    /// the requested file is correctly located in the given location.
    ///
    /// A panic in the function will cause DownloadResult::Err to be sent to the underlying
    /// libalpm (i.e. not undefined behaviour).
    ///
    /// TODO investigate whether safe to relax 'static bound
    pub unsafe fn fetch_function<F>(&mut self, func: F)
        where F: FnMut(&str, &str, bool) -> DownloadResult + Send + 'static
    {
        let mut cb = FETCH_CB.lock().unwrap();
        (*cb) = Some(Box::new(func));
        alpm_option_set_fetchcb(self.handle, Some(alpm_cb_fetch));
    }

    /// Clears the file download callback, falling back to built-in fetch functionality.
    pub fn clear_fetch_function(&mut self) {
        let mut cb = DLTOTAL_CB.lock().unwrap();
        (*cb) = None;
        unsafe { alpm_option_set_fetchcb(self.handle, None); }
    }

    /// Sets the function called when an event occurs
    pub fn event_function<F>(&mut self, func: F)
        where F: FnMut(Event) + Send + 'static
    {
        let mut cb = EVENT_CB.lock().unwrap();
        (*cb) = Some(Box::new(func));
        unsafe { alpm_option_set_eventcb(self.handle, Some(alpm_cb_event)); }
    }

    /// Clears the file download callback, falling back to built-in fetch functionality.
    pub fn clear_event_function(&mut self) {
        let mut cb = DLTOTAL_CB.lock().unwrap();
        (*cb) = None;
        unsafe { alpm_option_set_eventcb(self.handle, None); }
    }

    /// Sets the function called when a question needs answering (todo i think)
    pub fn question_function<F>(&mut self, func: F)
        where F: FnMut() + Send + 'static
    {
        unimplemented!()
    }

    /// Clears the function called when a question needs answering (todo i think)
    pub fn clear_question_function(&mut self) {
        unimplemented!()
    }

    /// Sets the function called to show operation progress
    pub fn progress_function<F>(&mut self, func: F)
        where F: FnMut() + Send + 'static
    {
        unimplemented!()
    }

    /// Clears the function called to show operation progress
    pub fn clear_progress_function(&mut self) {
        unimplemented!()
    }

    /// Get the root path used in this instance of alpm
    ///
    /// The api doesn't make clear the lifetime of the result, so I am conservative (same goes for
    /// db_path)
    pub fn root<'a>(&'a self) -> &'a str {
        let root = unsafe { CStr::from_ptr(alpm_option_get_root(self.handle)) };
        root.to_str().ok().expect("instance root path is not utf8")
    }

    /// Get the database path used in this instance of alpm
    pub fn db_path<'a>(&'a self) -> &'a str {
        let db_path = unsafe { CStr::from_ptr(alpm_option_get_dbpath(self.handle)) };
        db_path.to_str().ok().expect("instance db path is not utf8")
    }

    /// Get the lockfile path used in this instance of alpm
    pub fn lockfile<'a>(&'a self) -> &'a str {
        let lockfile = unsafe { CStr::from_ptr(alpm_option_get_lockfile(self.handle)) };
        lockfile.to_str().ok().expect("instance lockfile path is not utf8")
    }

    /// Gets a list of the cache directories in use by this instance of alpm
    pub fn cache_dirs(&self) -> Vec<&str> {
        unsafe {
            let cachedirs = alpm_option_get_cachedirs(self.handle);
            util::alpm_list_to_vec(cachedirs, |char_ptr| {
                CStr::from_ptr(char_ptr as *const c_char).to_str().unwrap()
            })
        }
    }

    /// Sets a list of the cache directories in use by this instance of alpm
    pub fn set_cache_dirs(&self) {
        unimplemented!()
    }

    /// Adds a cache directory for use by this instance of alpm
    pub fn add_cache_dir(&self) {
        unimplemented!()
    }

    /// Removes a cache directory in use by this instance of alpm
    pub fn remove_cache_dir(&self) {
        unimplemented!()
    }

    /// Gets a list of the hook directories in use by this instance of alpm
    pub fn hook_dirs(&self) {
        unimplemented!()
    }

    /// Sets a list of the hook directories in use by this instance of alpm
    pub fn set_hook_dirs(&self) {
        unimplemented!()
    }

    /// Adds a hook directory for use by this instance of alpm
    pub fn add_hook_dir(&self) {
        unimplemented!()
    }

    /// Removes a hook directory in use by this instance of alpm
    pub fn remove_hook_dir(&self) {
        unimplemented!()
    }

    /// Gets the log file location used by this instance of alpm.
    pub fn log_file(&self) {
        unimplemented!()
    }

    /// Sets the log file location used by this instance of alpm.
    pub fn set_log_file(&self) {
        unimplemented!()
    }

    /// Gets the path to alpm's GnuPG home directory
    pub fn gpg_dir(&self) {
        unimplemented!()
    }

    /// Sets the path to alpm's GnuPG home directory
    pub fn set_gpg_dir(&self) {
        unimplemented!()
    }

    /// Gets whether this instance of alpm should log events to syslog
    pub fn use_syslog(&self) {
        unimplemented!()
    }

    /// Sets whether this instance of alpm should log events to syslog
    pub fn set_use_syslog(&self) {
        unimplemented!()
    }

    /// Gets a list of the packages that should not be upgraded.
    pub fn no_upgrades(&self) {
        unimplemented!()
    }

    /// Sets a list of the packages that should not be upgraded.
    pub fn set_no_upgrades(&self) {
        unimplemented!()
    }

    /// Adds a package to the list that should not be upgraded.
    pub fn add_no_upgrade(&self) {
        unimplemented!()
    }

    /// Removes a package from the list that should not be upgraded.
    pub fn remove_no_upgrade(&self) {
        unimplemented!()
    }

    /// Gets a list of the packages that should be ignored.
    pub fn ignore_pkgs(&self) {
        unimplemented!()
    }

    /// Sets a list of the packages that should be ignored.
    pub fn set_ignore_pkgs(&self) {
        unimplemented!()
    }

    /// Adds a package to the list that should be ignored.
    pub fn add_ignore_pkg(&self) {
        unimplemented!()
    }

    /// Removes a package from the list that should be ignored.
    pub fn remove_ignore_pkg(&self) {
        unimplemented!()
    }

    /// Gets a list of the groups that should be ignored.
    pub fn ignore_groups(&self) {
        unimplemented!()
    }

    /// Sets a list of the groups that should be ignored.
    pub fn set_ignore_groups(&self) {
        unimplemented!()
    }

    /// Adds a group to the list that should be ignored.
    pub fn add_ignore_group(&self) {
        unimplemented!()
    }

    /// Removes a group from the list that should be ignored.
    pub fn remove_ignore_group(&self) {
        unimplemented!()
    }

    /// Gets a list of the dependencies that should be ignored by a sys-upgrade.
    pub fn assume_installed(&self) {
        unimplemented!()
    }

    /// Sets a list of the dependencies that should be ignored by a sys-upgrade.
    pub fn set_assume_installed(&self) {
        unimplemented!()
    }

    /// Adds a package to the list of dependencies that should be ignored by a sys-upgrade.
    pub fn add_assume_installed(&self) {
        unimplemented!()
    }

    /// Removes a package from the list of dependencies that should be ignored by a sys-upgrade.
    pub fn remove_assume_installed(&self) {
        unimplemented!()
    }

    /// Gets the targeted architecture.
    pub fn arch(&self) -> Option<&str> {
        unsafe {
            let arch = alpm_option_get_arch(self.handle);
            if arch.is_null() {
                None
            } else {
                Some(CStr::from_ptr(arch).to_str().ok()
                    .expect("targeted arch is not utf8"))
            }
        }
    }

    /// Sets the targeted architecture.
    pub fn set_arch(&mut self, arch: &str) -> AlpmResult<()> {
        let arch = CString::new(arch)?;
        let res = unsafe { alpm_option_set_arch(self.handle, arch.as_ptr()) };
        if res == 0 {
            Ok(())
        } else {
            Err(self.error().unwrap_or(Error::__Unknown))
        }
    }

    /// Gets the delta ratio
    pub fn delta_ratio(&self) -> f64 {
        unsafe { alpm_option_get_deltaratio(self.handle) }
    }

    /// Sets the targeted architecture
    pub fn set_delta_ratio(&mut self, r: f64) -> AlpmResult<()> {
        let res = unsafe { alpm_option_set_deltaratio(self.handle, r) };
        if res == 0 {
            Ok(())
        } else {
            Err(self.error().unwrap_or(Error::__Unknown))
        }
    }

    /// Gets whether alpm will check disk space before operations
    pub fn check_space(&self) -> bool {
        unsafe { alpm_option_get_checkspace(self.handle) != 0 }
    }

    /// Sets the targeted architecture
    pub fn set_check_space(&mut self, check: bool) -> AlpmResult<()> {
        let res = unsafe { alpm_option_set_checkspace(self.handle, if check { 1 } else { 0 }) };
        if res == 0 {
            Ok(())
        } else {
            Err(self.error().unwrap_or(Error::__Unknown))
        }
    }

    /// Gets the registered database extension used on the filesystem
    pub fn db_extension(&self) -> &str {
        unsafe {
            let ext = alpm_option_get_dbext(self.handle);
            assert!(!ext.is_null(), "Database extension should never be null");
            CStr::from_ptr(ext).to_str().ok().expect("Database extensions not valid utf8")
        }
    }

    /// Sets the targeted architecture
    pub fn set_db_extension(&mut self, ext: &str) -> AlpmResult<()> {
        let cstr = CString::new(ext)?;
        let res = unsafe { alpm_option_set_dbext(self.handle, cstr.as_ptr()) };
        if res == 0 {
            Ok(())
        } else {
            Err(self.error().unwrap_or(Error::__Unknown))
        }
    }

    /// Gets the default signing level
    pub fn default_sign_level(&self) -> SigLevel {
        unsafe { alpm_option_get_default_siglevel(self.handle).into() }
    }

    /// Sets the default signing level
    pub fn set_default_sign_level(&mut self, s: SigLevel) -> AlpmResult<()> {
        let res = unsafe { alpm_option_set_default_siglevel(self.handle, s.into()) };
        if res == 0 {
            Ok(())
        } else {
            Err(self.error().unwrap_or(Error::__Unknown))
        }
    }

    /// Gets the default signing level
    pub fn local_file_sign_level(&self) -> SigLevel {
        unsafe { alpm_option_get_local_file_siglevel(self.handle).into() }
    }

    /// Sets the default signing level
    pub fn set_local_file_sign_level(&mut self, s: SigLevel) -> AlpmResult<()> {
        let res = unsafe { alpm_option_set_local_file_siglevel(self.handle, s.into()) };
        if res == 0 {
            Ok(())
        } else {
            Err(self.error().unwrap_or(Error::__Unknown))
        }
    }

    /// Gets the default signing level
    pub fn remote_file_sign_level(&self) -> SigLevel {
        unsafe { alpm_option_get_remote_file_siglevel(self.handle).into() }
    }

    /// Sets the default signing level
    pub fn set_remote_file_sign_level(&mut self, s: SigLevel) -> AlpmResult<()> {
        let res = unsafe { alpm_option_set_remote_file_siglevel(self.handle, s.into()) };
        if res == 0 {
            Ok(())
        } else {
            Err(self.error().unwrap_or(Error::__Unknown))
        }
    }

    /// Get the local database instance.
    pub fn local_db<'a>(&'a self) -> Db<'a> {
        unsafe { Db::new(alpm_get_localdb(self.handle), self) }
    }

    /// Get a list of remote databases registered.
    pub fn sync_dbs<'a>(&'a self) -> Vec<Db<'a>> {
        //use std::error::Error;
        unsafe {
            let raw_list = alpm_get_syncdbs(self.handle);
            //println!("{:?}", raw_list);
            //println!("error: {:?}", self.error().unwrap().description());
            util::alpm_list_to_vec(raw_list, |ptr| {
                Db::new(ptr as *const Struct_alpm_db, &self)
            })
        }
    }

    /// Register a sync db (remote db). You will need to attach servers to the db to be able to
    /// sync
    pub fn register_sync_db<'a>(&'a mut self, treename: &str, level: SigLevel) -> AlpmResult<Db<'a>> {
        unsafe {
            let db = alpm_register_syncdb(self.handle,
                                          (CString::new(treename)?).as_ptr(),
                                          level.into());
            if db.is_null() {
                Err(self.error().unwrap_or(Error::__Unknown))
            } else {
                Ok(Db::new(db, self))
            }
        }
    }

    /*
    /// Unregister all sync dbs.
    ///
    /// # Safety
    /// There must not be any remaining Db instances, as these will be de-allocated.
    pub unsafe fn unregister_all_sync_dbs(&self) -> AlpmResult<()> {
        let res = alpm_unregister_all_syncdbs(self.handle);
        if res == 0 {
            Ok(())
        } else {
            Err(self.error().unwrap_or(Error::__Unknown))
        }
    }
    */

    /// Start a package modification transaction
    ///
    /// This locks the database (creates the lockfile).
    pub fn init_transaction<'a>(&'a mut self, flags: TransactionFlags)
        -> AlpmResult<Transaction<'a, Initialized>>
    {
        let res = unsafe { alpm_trans_init(self.handle, flags.into()) };
        if res == 0 {
            Ok(Transaction {
                alpm: self,
                _state: PhantomData,
            })
        } else {
            // Make sure we release our db lock
            unsafe { alpm_trans_release(self.handle) };
            Err(self.error().unwrap_or(Error::__Unknown))
        }
    }
}

impl Drop for Alpm {
    fn drop(&mut self) {
        unsafe { alpm_release(self.handle); }
    }
}

/// A state marker for before a transaction is prepared
pub enum Initialized {}

/// A state marker for before a transaction is committed, but after it is prepared
pub enum Prepared {}

/// A transaction of package operations
///
/// Only certain state transitions are valid TODO model this how hyper crate models response state
///
/// Consumes an Alpm instance as only 1 transaction can be performed at a time. Use `commit` or
/// `rollback` to recover the Alpm instance.
pub struct Transaction<'a, S: Any = Initialized> {
    alpm: &'a mut Alpm,
    _state: PhantomData<S>
}

// This removes the lockfile to make sure future alpm changes can happen
impl<'a, S: Any> Drop for Transaction<'a, S> {
    fn drop(&mut self) {
        unsafe { alpm_trans_release(self.alpm.handle) };
    }
}

impl<'a, S: Any> Transaction<'a, S> {

    /// Returns the flags for the current transaction.
    pub fn flags(&self) -> TransactionFlags {
        unsafe { alpm_trans_get_flags(self.alpm.handle).into() }
    }

    /// Deconstructs the transaction without dropping. Internal only. From hyper.
    fn deconstruct(self) -> &'a mut Alpm {
        unsafe {
            let alpm = ptr::read(&self.alpm);
            mem::forget(self);
            alpm
        }
    }

}

impl<'a> Transaction<'a, Initialized> {

    /// Prepares a transaction for committing.
    ///
    ///  - Checks arch of added packages (fails if arch is wrong for any of them).
    ///  - Checks package removal (todo how does this work?)
    ///  - Reorders package addition and removal into correct dependency order. Emits warning on
    ///    circular dependency.
    pub fn prepare(mut self)
        -> AlpmResult<Transaction<'a, Prepared>>
    {
        unsafe {
            let mut p: *mut alpm_list_t = ptr::null_mut();
            let res = alpm_trans_prepare(self.alpm.handle, &mut p as *mut _);
            if res == 0 {
                let alpm = self.deconstruct();
                Ok(Transaction {
                    alpm: alpm,
                    _state: PhantomData
                })
            } else {
                Err(self.alpm.error().unwrap_or(Error::__Unknown))
            }
        }
    }

    /// Adds a system upgrade to this transaction.
    pub fn sys_upgrade(&self, enable_downgrade: bool) -> AlpmResult<()> {
        unsafe {
            let res = alpm_sync_sysupgrade(self.alpm.handle, enable_downgrade as libc::c_int);
            if res == 0 {
                Ok(())
            } else {
                Err(self.alpm.error().unwrap_or(Error::__Unknown))
            }
        }
    }

    /// Adds a new package to system in this transaction.
    pub fn add_package(&self, pkg: &PackageRef) -> AlpmResult<()> {
        unimplemented!()
    }

    /// Removes a package from the system in this transaction.
    pub fn remove_package(&self, pkg: &PackageRef) -> AlpmResult<()> {
        unimplemented!()
    }
}

impl<'a> Transaction<'a, Prepared> {

    /// Commits the transaction and returns the alpm instance. TODO conflict type
    ///
    ///  - Download required new packages
    ///  - Check downloaded packages for integrity
    ///  - Synchronize filesystem
    ///
    /// TODO find out how this long-run op works (I guess that this blocks, but another thread can
    /// call interrupt?)
    pub fn commit(self) -> AlpmResult<()> {
        use std::ptr;
        unsafe {
            let mut p: *mut alpm_list_t = ptr::null_mut();
            let res = alpm_trans_commit(self.alpm.handle, &mut p as *mut _);
            if res == 0 {
                Ok(())
            } else {
                Err(self.alpm.error().unwrap_or(Error::__Unknown))
            }
        }
    }

    /// Gets packages added by the current transaction.
    pub fn added_packages(&'a self) -> Vec<&'a PackageRef> {
        unimplemented!()
    }

    /// Gets packages removed by the current transaction.
    pub fn removed_packages(&'a self) -> Vec<&'a PackageRef> {
        unimplemented!()
    }

}


/// Get the version of the attached libalpm
pub fn version() -> &'static str {
    unsafe {
        let v = CStr::from_ptr(alpm_version());
        v.to_str().ok().expect("For some reason the libalpm version is not utf8")
    }
}

/// Get the capabilities of the attached libalpm
pub fn capabilities() -> Caps {
    unsafe { alpm_capabilities().into() }
}

#[derive(Default, Debug, PartialEq, Eq, Copy, Clone)]
pub struct TransactionFlags {
    /// Ignore dependency checks
    no_deps: bool,
    /// Ignore file conflicts and overwrite files
    force: bool,
    /// Delete files even if they are tagged as backup
    no_save: bool,
    /// Ignore version numbers when checking dependencies
    no_dep_version: bool,
    /// Remove also any packages depending on a package being removed
    cascade: bool,
    /// Remove packages and their unneeded deps (not explicitally installed)
    recurse: bool,
    /// Modify database but do not commit changes to filesystem
    db_only: bool,
    /// Mark all installed packages as dependencies.
    all_deps: bool,
    /// Only download packages and do not actually install.
    download_only: bool,
    /// Do not execute install scriptlets after installing
    no_scriptlet: bool,
    /// Ignore dependency conflicts
    no_conflicts: bool,
    /// Do not install a package if it is already installed and up to date
    needed: bool,
    /// Mark all installed packages as explicitally requested.
    all_explicit: bool,
    /// Do not remove a package if it is needed by another one.
    unneeded: bool,
    /// Remove also explicitly installed unneeded deps (use with `recurse: true`)
    recurse_all: bool,
    /// Do not lock the database during the operation.
    no_lock: bool,
}

impl Into<u32> for TransactionFlags {
    fn into(self) -> u32 {
        let mut acc = 0;
        if self.no_deps {
            acc |= ALPM_TRANS_FLAG_NODEPS;
        }
        if self.force {
            acc |= ALPM_TRANS_FLAG_FORCE;
        }
        if self.no_save {
            acc |= ALPM_TRANS_FLAG_NOSAVE;
        }
        if self.no_dep_version {
            acc |= ALPM_TRANS_FLAG_NODEPVERSION;
        }
        if self.cascade {
            acc |= ALPM_TRANS_FLAG_CASCADE;
        }
        if self.recurse {
            acc |= ALPM_TRANS_FLAG_RECURSE;
        }
        if self.db_only {
            acc |= ALPM_TRANS_FLAG_DBONLY;
        }
        if self.all_deps {
            acc |= ALPM_TRANS_FLAG_ALLDEPS;
        }
        if self.download_only {
            acc |= ALPM_TRANS_FLAG_DOWNLOADONLY;
        }
        if self.no_scriptlet {
            acc |= ALPM_TRANS_FLAG_NOSCRIPTLET;
        }
        if self.no_conflicts {
            acc |= ALPM_TRANS_FLAG_NOCONFLICTS;
        }
        if self.needed {
            acc |= ALPM_TRANS_FLAG_NEEDED;
        }
        if self.all_explicit {
            acc |= ALPM_TRANS_FLAG_ALLEXPLICIT;
        }
        if self.unneeded {
            acc |= ALPM_TRANS_FLAG_UNNEEDED;
        }
        if self.recurse_all {
            acc |= ALPM_TRANS_FLAG_RECURSEALL;
        }
        if self.no_lock {
            acc |= ALPM_TRANS_FLAG_NOLOCK;
        }
        acc
    }
}

impl From<u32> for TransactionFlags {
    fn from(from: u32) -> TransactionFlags {
        TransactionFlags {
            no_deps: from & ALPM_TRANS_FLAG_NODEPS != 0,
            force: from & ALPM_TRANS_FLAG_FORCE != 0,
            no_save: from & ALPM_TRANS_FLAG_NOSAVE != 0,
            no_dep_version: from & ALPM_TRANS_FLAG_NODEPVERSION != 0,
            cascade: from & ALPM_TRANS_FLAG_CASCADE != 0,
            recurse: from & ALPM_TRANS_FLAG_RECURSE != 0,
            db_only: from & ALPM_TRANS_FLAG_DBONLY != 0,
            all_deps: from & ALPM_TRANS_FLAG_ALLDEPS != 0,
            download_only: from & ALPM_TRANS_FLAG_DOWNLOADONLY != 0,
            no_scriptlet: from & ALPM_TRANS_FLAG_NOSCRIPTLET != 0,
            no_conflicts: from & ALPM_TRANS_FLAG_NOCONFLICTS != 0,
            needed: from & ALPM_TRANS_FLAG_NEEDED != 0,
            all_explicit: from & ALPM_TRANS_FLAG_ALLEXPLICIT != 0,
            unneeded: from & ALPM_TRANS_FLAG_UNNEEDED != 0,
            recurse_all: from & ALPM_TRANS_FLAG_RECURSEALL != 0,
            no_lock: from & ALPM_TRANS_FLAG_NOLOCK != 0,
        }
    }
}

#[test]
fn test_transaction_flags() {
    let t: TransactionFlags = Default::default();
    // (my) sanity check that deriving bool = false
    assert!(!t.no_lock);
}
