use librustdesk::*;

#[cfg(not(target_os = "macos"))]
fn main() {}

#[cfg(target_os = "macos")]
fn main() {
    crate::common::load_custom_client();
    
    // Initialize DEFAULT_SETTINGS with UserDefaultConfig default values
    // This ensures UI can display correct default states for options
    hbb_common::config::init_default_settings();
    
    hbb_common::init_log(false, "service");
    crate::start_os_service();
}
