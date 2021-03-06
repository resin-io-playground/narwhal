#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate lazy_static;
extern crate openssl;
extern crate regex;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate url;

pub mod httpstream;
pub mod tcp;
pub mod unix;
pub mod tls;

pub mod errors;
pub mod types;
pub mod network;
pub mod utils;
pub mod queryparameters;

pub mod engine;
pub mod images;
pub mod containers;

// Export main types to top level of the crate
pub use queryparameters::QueryFilter;
pub use queryparameters::QueryParameters;
pub use types::Client;
