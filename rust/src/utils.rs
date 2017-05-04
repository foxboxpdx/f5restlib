// Utility functions
use regex::Regex;
use hyper::client::Response;
use hyper::status::StatusCode;

pub fn validate_name(name: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"[^\w\.-]").unwrap();
    }
    RE.is_match(name)
}

pub fn validate_ipport(ipprt: &str) -> bool {
    true
}

pub fn validate_return_code(response: &Response) -> bool {
    true
}

