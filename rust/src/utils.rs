// Utility functions
use regex::Regex;
use hyper::client::Response;
use std::net::SocketAddr;
use std::io::Read;

pub fn validate_name(name: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"[^\w\.-]").unwrap();
    }
    RE.is_match(name)
}

pub fn validate_ipport(ipport: &str) -> bool {
    // Unlike Ruby, we can use SocketAddr to parse a string into an IP:Port pair
    // so no need to split(':') or any of that nonsense.
    let addr: SocketAddr = ipport.parse().expect("Bad IP:Port pair");
    true
}

pub fn validate_return_code(response: &mut Response) -> bool {
    // This was a lot fancier in the Ruby version.  I suppose we only really
    // need to check for 200 or not 200 and spit out the code/body.
    if response.status.is_success() {
        return true
    } else {
        let stat = response.status;
        let mut why = String::new();
        response.read_to_string(&mut why).expect("Can't read response?!");
        println!("Received error {}: {:?}", stat, why);
        return false
    }
}

