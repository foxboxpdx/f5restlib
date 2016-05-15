extern crate f5restlib;
extern crate serde;
extern crate serde_json;

use f5restlib::connection::F5Connection;
use f5restlib::types::VirtualRoot;
use std::io::Read;

fn main() {
    let conn = F5Connection::new("user", "pass", "https://uri");
    let mut bar = conn.get("/mgmt/tm/ltm/virtual/");
    let mut buffer = String::new();
    bar.read_to_string(&mut buffer).unwrap();
    let stuff = serde_json::de::from_str::<VirtualRoot>(&buffer).unwrap();
    println!("{:?}", stuff);
}
