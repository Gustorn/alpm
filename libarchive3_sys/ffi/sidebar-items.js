initSidebarItems({"constant":[["AE_IFBLK",""],["AE_IFCHR",""],["AE_IFDIR",""],["AE_IFIFO",""],["AE_IFLNK",""],["AE_IFMT",""],["AE_IFREG",""],["AE_IFSOCK",""],["ARCHIVE_EOF",""],["ARCHIVE_EXTRACT_ACL",""],["ARCHIVE_EXTRACT_CLEAR_NOCHANGE_FFLAGS",""],["ARCHIVE_EXTRACT_FFLAGS",""],["ARCHIVE_EXTRACT_HFS_COMPRESSION_FORCED",""],["ARCHIVE_EXTRACT_MAC_METADATA",""],["ARCHIVE_EXTRACT_NO_AUTODIR",""],["ARCHIVE_EXTRACT_NO_HFS_COMPRESSION",""],["ARCHIVE_EXTRACT_NO_OVERWRITE",""],["ARCHIVE_EXTRACT_NO_OVERWRITE_NEWER",""],["ARCHIVE_EXTRACT_OWNER",""],["ARCHIVE_EXTRACT_PERM",""],["ARCHIVE_EXTRACT_SECURE_NOABSOLUTEPATHS",""],["ARCHIVE_EXTRACT_SECURE_NODOTDOT",""],["ARCHIVE_EXTRACT_SECURE_SYMLINKS",""],["ARCHIVE_EXTRACT_SPARSE",""],["ARCHIVE_EXTRACT_TIME",""],["ARCHIVE_EXTRACT_UNLINK",""],["ARCHIVE_EXTRACT_XATTR",""],["ARCHIVE_FAILED",""],["ARCHIVE_FATAL",""],["ARCHIVE_OK",""],["ARCHIVE_RETRY",""],["ARCHIVE_WARN",""]],"enum":[["Struct_archive",""],["Struct_archive_acl",""],["Struct_archive_entry",""],["Struct_archive_entry_linkresolver",""]],"fn":[["archive_clear_error",""],["archive_compression",""],["archive_compression_name",""],["archive_copy_error",""],["archive_entry_acl",""],["archive_entry_acl_add_entry",""],["archive_entry_acl_add_entry_w",""],["archive_entry_acl_clear",""],["archive_entry_acl_count",""],["archive_entry_acl_next",""],["archive_entry_acl_next_w",""],["archive_entry_acl_reset",""],["archive_entry_acl_text",""],["archive_entry_acl_text_w",""],["archive_entry_atime",""],["archive_entry_atime_is_set",""],["archive_entry_atime_nsec",""],["archive_entry_birthtime",""],["archive_entry_birthtime_is_set",""],["archive_entry_birthtime_nsec",""],["archive_entry_clear",""],["archive_entry_clone",""],["archive_entry_copy_fflags_text",""],["archive_entry_copy_fflags_text_w",""],["archive_entry_copy_gname",""],["archive_entry_copy_gname_w",""],["archive_entry_copy_hardlink",""],["archive_entry_copy_hardlink_w",""],["archive_entry_copy_link",""],["archive_entry_copy_link_w",""],["archive_entry_copy_mac_metadata",""],["archive_entry_copy_pathname",""],["archive_entry_copy_pathname_w",""],["archive_entry_copy_sourcepath",""],["archive_entry_copy_sourcepath_w",""],["archive_entry_copy_symlink",""],["archive_entry_copy_symlink_w",""],["archive_entry_copy_uname",""],["archive_entry_copy_uname_w",""],["archive_entry_ctime",""],["archive_entry_ctime_is_set",""],["archive_entry_ctime_nsec",""],["archive_entry_dev",""],["archive_entry_dev_is_set",""],["archive_entry_devmajor",""],["archive_entry_devminor",""],["archive_entry_fflags",""],["archive_entry_fflags_text",""],["archive_entry_filetype",""],["archive_entry_free",""],["archive_entry_gid",""],["archive_entry_gname",""],["archive_entry_gname_w",""],["archive_entry_hardlink",""],["archive_entry_hardlink_w",""],["archive_entry_ino",""],["archive_entry_ino64",""],["archive_entry_ino_is_set",""],["archive_entry_linkify",""],["archive_entry_linkresolver_free",""],["archive_entry_linkresolver_new",""],["archive_entry_linkresolver_set_strategy",""],["archive_entry_mac_metadata",""],["archive_entry_mode",""],["archive_entry_mtime",""],["archive_entry_mtime_is_set",""],["archive_entry_mtime_nsec",""],["archive_entry_new",""],["archive_entry_new2",""],["archive_entry_nlink",""],["archive_entry_partial_links",""],["archive_entry_pathname",""],["archive_entry_pathname_w",""],["archive_entry_perm",""],["archive_entry_rdev",""],["archive_entry_rdevmajor",""],["archive_entry_rdevminor",""],["archive_entry_set_atime",""],["archive_entry_set_birthtime",""],["archive_entry_set_ctime",""],["archive_entry_set_dev",""],["archive_entry_set_devmajor",""],["archive_entry_set_devminor",""],["archive_entry_set_fflags",""],["archive_entry_set_filetype",""],["archive_entry_set_gid",""],["archive_entry_set_gname",""],["archive_entry_set_hardlink",""],["archive_entry_set_ino",""],["archive_entry_set_ino64",""],["archive_entry_set_link",""],["archive_entry_set_mode",""],["archive_entry_set_mtime",""],["archive_entry_set_nlink",""],["archive_entry_set_pathname",""],["archive_entry_set_perm",""],["archive_entry_set_rdev",""],["archive_entry_set_rdevmajor",""],["archive_entry_set_rdevminor",""],["archive_entry_set_size",""],["archive_entry_set_symlink",""],["archive_entry_set_uid",""],["archive_entry_set_uname",""],["archive_entry_size",""],["archive_entry_size_is_set",""],["archive_entry_sourcepath",""],["archive_entry_sourcepath_w",""],["archive_entry_sparse_add_entry",""],["archive_entry_sparse_clear",""],["archive_entry_sparse_count",""],["archive_entry_sparse_next",""],["archive_entry_sparse_reset",""],["archive_entry_strmode",""],["archive_entry_symlink",""],["archive_entry_symlink_w",""],["archive_entry_uid",""],["archive_entry_uname",""],["archive_entry_uname_w",""],["archive_entry_unset_atime",""],["archive_entry_unset_birthtime",""],["archive_entry_unset_ctime",""],["archive_entry_unset_mtime",""],["archive_entry_unset_size",""],["archive_entry_update_gname_utf8",""],["archive_entry_update_hardlink_utf8",""],["archive_entry_update_link_utf8",""],["archive_entry_update_pathname_utf8",""],["archive_entry_update_symlink_utf8",""],["archive_entry_update_uname_utf8",""],["archive_entry_xattr_add_entry",""],["archive_entry_xattr_clear",""],["archive_entry_xattr_count",""],["archive_entry_xattr_next",""],["archive_entry_xattr_reset",""],["archive_errno",""],["archive_error_string",""],["archive_file_count",""],["archive_filter_bytes",""],["archive_filter_code",""],["archive_filter_count",""],["archive_filter_name",""],["archive_format",""],["archive_format_name",""],["archive_match_exclude_entry",""],["archive_match_exclude_pattern",""],["archive_match_exclude_pattern_from_file",""],["archive_match_exclude_pattern_from_file_w",""],["archive_match_exclude_pattern_w",""],["archive_match_excluded",""],["archive_match_free",""],["archive_match_include_date",""],["archive_match_include_date_w",""],["archive_match_include_file_time",""],["archive_match_include_file_time_w",""],["archive_match_include_gid",""],["archive_match_include_gname",""],["archive_match_include_gname_w",""],["archive_match_include_pattern",""],["archive_match_include_pattern_from_file",""],["archive_match_include_pattern_from_file_w",""],["archive_match_include_pattern_w",""],["archive_match_include_time",""],["archive_match_include_uid",""],["archive_match_include_uname",""],["archive_match_include_uname_w",""],["archive_match_new",""],["archive_match_owner_excluded",""],["archive_match_path_excluded",""],["archive_match_path_unmatched_inclusions",""],["archive_match_path_unmatched_inclusions_next",""],["archive_match_path_unmatched_inclusions_next_w",""],["archive_match_time_excluded",""],["archive_position_compressed",""],["archive_position_uncompressed",""],["archive_read_add_callback_data",""],["archive_read_append_callback_data",""],["archive_read_append_filter",""],["archive_read_append_filter_program",""],["archive_read_append_filter_program_signature",""],["archive_read_close",""],["archive_read_data",""],["archive_read_data_block",""],["archive_read_data_into_fd",""],["archive_read_data_skip",""],["archive_read_disk_can_descend",""],["archive_read_disk_current_filesystem",""],["archive_read_disk_current_filesystem_is_remote",""],["archive_read_disk_current_filesystem_is_synthetic",""],["archive_read_disk_descend",""],["archive_read_disk_gname",""],["archive_read_disk_new",""],["archive_read_disk_open",""],["archive_read_disk_open_w",""],["archive_read_disk_set_atime_restored",""],["archive_read_disk_set_behavior",""],["archive_read_disk_set_gname_lookup",""],["archive_read_disk_set_matching",""],["archive_read_disk_set_metadata_filter_callback",""],["archive_read_disk_set_standard_lookup",""],["archive_read_disk_set_symlink_hybrid",""],["archive_read_disk_set_symlink_logical",""],["archive_read_disk_set_symlink_physical",""],["archive_read_disk_set_uname_lookup",""],["archive_read_disk_uname",""],["archive_read_extract",""],["archive_read_extract2",""],["archive_read_extract_set_progress_callback",""],["archive_read_extract_set_skip_file",""],["archive_read_finish",""],["archive_read_free",""],["archive_read_header_position",""],["archive_read_new",""],["archive_read_next_header",""],["archive_read_next_header2",""],["archive_read_open",""],["archive_read_open1",""],["archive_read_open2",""],["archive_read_open_FILE",""],["archive_read_open_fd",""],["archive_read_open_file",""],["archive_read_open_filename",""],["archive_read_open_filename_w",""],["archive_read_open_filenames",""],["archive_read_open_memory",""],["archive_read_open_memory2",""],["archive_read_prepend_callback_data",""],["archive_read_set_callback_data",""],["archive_read_set_callback_data2",""],["archive_read_set_close_callback",""],["archive_read_set_filter_option",""],["archive_read_set_format",""],["archive_read_set_format_option",""],["archive_read_set_open_callback",""],["archive_read_set_option",""],["archive_read_set_options",""],["archive_read_set_read_callback",""],["archive_read_set_seek_callback",""],["archive_read_set_skip_callback",""],["archive_read_set_switch_callback",""],["archive_read_support_compression_all",""],["archive_read_support_compression_bzip2",""],["archive_read_support_compression_compress",""],["archive_read_support_compression_gzip",""],["archive_read_support_compression_lzip",""],["archive_read_support_compression_lzma",""],["archive_read_support_compression_none",""],["archive_read_support_compression_program",""],["archive_read_support_compression_program_signature",""],["archive_read_support_compression_rpm",""],["archive_read_support_compression_uu",""],["archive_read_support_compression_xz",""],["archive_read_support_filter_all",""],["archive_read_support_filter_bzip2",""],["archive_read_support_filter_compress",""],["archive_read_support_filter_grzip",""],["archive_read_support_filter_gzip",""],["archive_read_support_filter_lrzip",""],["archive_read_support_filter_lzip",""],["archive_read_support_filter_lzma",""],["archive_read_support_filter_lzop",""],["archive_read_support_filter_none",""],["archive_read_support_filter_program",""],["archive_read_support_filter_program_signature",""],["archive_read_support_filter_rpm",""],["archive_read_support_filter_uu",""],["archive_read_support_filter_xz",""],["archive_read_support_format_7zip",""],["archive_read_support_format_all",""],["archive_read_support_format_ar",""],["archive_read_support_format_by_code",""],["archive_read_support_format_cab",""],["archive_read_support_format_cpio",""],["archive_read_support_format_empty",""],["archive_read_support_format_gnutar",""],["archive_read_support_format_iso9660",""],["archive_read_support_format_lha",""],["archive_read_support_format_mtree",""],["archive_read_support_format_rar",""],["archive_read_support_format_raw",""],["archive_read_support_format_tar",""],["archive_read_support_format_xar",""],["archive_read_support_format_zip",""],["archive_seek_data",""],["archive_set_error",""],["archive_version_number",""],["archive_version_string",""],["archive_write_add_filter",""],["archive_write_add_filter_b64encode",""],["archive_write_add_filter_by_name",""],["archive_write_add_filter_bzip2",""],["archive_write_add_filter_compress",""],["archive_write_add_filter_grzip",""],["archive_write_add_filter_gzip",""],["archive_write_add_filter_lrzip",""],["archive_write_add_filter_lzip",""],["archive_write_add_filter_lzma",""],["archive_write_add_filter_lzop",""],["archive_write_add_filter_none",""],["archive_write_add_filter_program",""],["archive_write_add_filter_uuencode",""],["archive_write_add_filter_xz",""],["archive_write_close",""],["archive_write_data",""],["archive_write_data_block",""],["archive_write_disk_gid",""],["archive_write_disk_new",""],["archive_write_disk_set_group_lookup",""],["archive_write_disk_set_options",""],["archive_write_disk_set_skip_file",""],["archive_write_disk_set_standard_lookup",""],["archive_write_disk_set_user_lookup",""],["archive_write_disk_uid",""],["archive_write_fail",""],["archive_write_finish",""],["archive_write_finish_entry",""],["archive_write_free",""],["archive_write_get_bytes_in_last_block",""],["archive_write_get_bytes_per_block",""],["archive_write_header",""],["archive_write_new",""],["archive_write_open",""],["archive_write_open_FILE",""],["archive_write_open_fd",""],["archive_write_open_file",""],["archive_write_open_filename",""],["archive_write_open_filename_w",""],["archive_write_open_memory",""],["archive_write_set_bytes_in_last_block",""],["archive_write_set_bytes_per_block",""],["archive_write_set_compression_bzip2",""],["archive_write_set_compression_compress",""],["archive_write_set_compression_gzip",""],["archive_write_set_compression_lzip",""],["archive_write_set_compression_lzma",""],["archive_write_set_compression_none",""],["archive_write_set_compression_program",""],["archive_write_set_compression_xz",""],["archive_write_set_filter_option",""],["archive_write_set_format",""],["archive_write_set_format_7zip",""],["archive_write_set_format_ar_bsd",""],["archive_write_set_format_ar_svr4",""],["archive_write_set_format_by_name",""],["archive_write_set_format_cpio",""],["archive_write_set_format_cpio_newc",""],["archive_write_set_format_gnutar",""],["archive_write_set_format_iso9660",""],["archive_write_set_format_mtree",""],["archive_write_set_format_mtree_classic",""],["archive_write_set_format_option",""],["archive_write_set_format_pax",""],["archive_write_set_format_pax_restricted",""],["archive_write_set_format_shar",""],["archive_write_set_format_shar_dump",""],["archive_write_set_format_ustar",""],["archive_write_set_format_v7tar",""],["archive_write_set_format_xar",""],["archive_write_set_format_zip",""],["archive_write_set_option",""],["archive_write_set_options",""],["archive_write_set_skip_file",""],["archive_write_zip_set_compression_deflate",""],["archive_write_zip_set_compression_store",""]],"type":[["archive_close_callback",""],["archive_open_callback",""],["archive_read_callback",""],["archive_seek_callback",""],["archive_skip_callback",""],["archive_switch_callback",""],["archive_write_callback",""]]});