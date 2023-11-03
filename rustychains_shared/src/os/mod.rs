cfg_if::cfg_if! {
    if #[cfg(target_os = "macos")] {
        mod darwin;
        use darwin as current_os;        
    }
}


pub const DYLD_INSERT_LIBRARIES_ENV_NAME: &str = current_os::DYLD_INSERT_LIBRARIES_ENV_NAME;