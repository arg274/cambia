#[cfg(debug_assertions)]
pub static DEFAULT_PORT: &str = "3031";
#[cfg(not(debug_assertions))]
pub static DEFAULT_PORT: &str = "3030";
// TODO: Make this configurable
pub static RIP_LOG_DIR: &str = "rip_logs";