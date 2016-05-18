extern crate f5restlib;
extern crate serde;
extern crate serde_json;

use f5restlib::connection::F5Connection;
use f5restlib::types::{ VirtualRoot, PoolRoot, ClientRoot, CertRoot, KeyRoot };
use std::io::Read;
use std::env;

fn main() {
    // Read data from env
    let user = env::var("F5USER").unwrap();
    let pass = env::var("F5PASS").unwrap();
    let uri  = env::var("F5URI").unwrap();
    let conn = F5Connection::new(&user, &pass, &uri);
    let mut bar = conn.get("/mgmt/tm/ltm/pool/");
    let mut buffer = String::new();
    match bar.read_to_string(&mut buffer) {
        Ok(foo) => println!("Read {} bytes.", foo),
        Err(e) => println!("Error: {:?}", e)
    }
    match serde_json::de::from_str::<PoolRoot>(&buffer) {
        Ok(foo) => println!("{:?}", foo),
        Err(e)  => println!("Bad data: {}", e)
    }
    bar = conn.get("/mgmt/tm/ltm/virtual/");
    buffer.clear();
    match bar.read_to_string(&mut buffer) {
        Ok(foo) => println!("Read {} bytes.", foo),
        Err(e) => println!("Error: {:?}", e)
    }
    match serde_json::de::from_str::<VirtualRoot>(&buffer) {
        Ok(foo) => println!("{:?}", foo),
        Err(e) => println!("Bad data: {}", e)
    }
    bar = conn.get("/mgmt/tm/ltm/profile/client-ssl/");
    buffer.clear();
    match bar.read_to_string(&mut buffer) {
        Ok(foo) => println!("Read {} bytes.", foo),
        Err(e) => println!("Error: {:?}", e)
    }
    match serde_json::de::from_str::<ClientRoot>(&buffer) {
        Ok(foo) => println!("{:?}", foo),
        Err(e) => println!("Bad data: {}", e)
    }
    bar = conn.get("/mgmt/tm/sys/crypto/key/");
    buffer.clear();
    match bar.read_to_string(&mut buffer) {
        Ok(foo) => println!("Read {} bytes.", foo),
        Err(e) => println!("Error: {:?}", e)
    }
    match serde_json::de::from_str::<KeyRoot>(&buffer) {
        Ok(foo) => println!("{:?}", foo),
        Err(e) => println!("Bad data: {}", e)
    }
    bar = conn.get("/mgmt/tm/sys/crypto/cert/");
    buffer.clear();
    match bar.read_to_string(&mut buffer) {
        Ok(foo) => println!("Read {} bytes.", foo),
        Err(e) => println!("Error: {:?}", e)
    }
    match serde_json::de::from_str::<CertRoot>(&buffer) {
        Ok(foo) => println!("{:?}", foo),
        Err(e) => println!("Bad data: {}", e)
    }
}
