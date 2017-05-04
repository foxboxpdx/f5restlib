#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate serde_derive;

extern crate hyper;
extern crate core;
extern crate serde;
extern crate serde_json;
extern crate regex;

// F5 Modules
pub mod types;
pub mod vip;
pub mod pool;
pub mod cert;
pub mod key;
pub mod pkcs12;
pub mod clientssl;
pub mod connection;
pub mod cli;
pub mod utils;
