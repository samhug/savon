#[macro_use]
extern crate log;
extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;

// reexport dependencies so they can be used from the generated code
// without requiring more imports
#[doc(hidden)]
pub mod internal {
    pub use chrono;
    pub use reqwest;
    pub use xmltree;
}

mod error;
pub mod gen;
pub mod http;
pub mod literal;
pub mod rpser;
pub mod wsdl;

pub use error::*;

// Initialize logger for tests so we can set RUST_LOG for debugging
#[cfg(test)]
#[ctor::ctor]
fn init() {
    env_logger::init();
}
