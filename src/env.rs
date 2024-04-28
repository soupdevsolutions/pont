pub const CARGO_PACKAGE_VERSION: &str = "CARGO_PKG_VERSION";

pub fn get_env_variable(name: &str) -> Option<String> {
    std::env::var(name).ok()
}
