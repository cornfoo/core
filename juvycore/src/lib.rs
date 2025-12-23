pub mod models;

pub use models::*;

uniffi::setup_scaffolding!("juvycore");
static LIB_VERSION: &str = env!("CARGO_PKG_VERSION");

#[uniffi::export]
pub fn lib_version() -> String {
    LIB_VERSION.to_string()
}
