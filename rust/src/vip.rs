use std::collections::HashMap;
use connection::F5Connection;
use types::F5Virtual;
use types::Profiles;

impl F5Virtual {
    pub fn new(n: &str, pt: &str, d: &str, pl: &str, sat: HashMap<String, String>, pf: Profiles) -> F5Virtual {
        F5Virtual { name: n.to_string(),
                    partition: pt.to_string(),
                    destination: d.to_string(),
                    pool: Some(pl.to_string()),
                    snat: sat,
                    profiles: pf }
    }

    pub fn from_yaml(data: &str) -> bool {
        true
    }

    pub fn create(&self, conn: &F5Connection) -> bool {
        true
    }

    pub fn collide(conn: &F5Connection, name: &str) -> bool {
        true
    }

    pub fn delete(&self, conn: &F5Connection) -> bool {
        true
    }

    pub fn delta(&self, conn: &F5Connection) -> bool {
        true
    }

    pub fn fetch(conn: &F5Connection, partition: &str, name: &str) -> bool {
        // This will actually return an F5Virtual
        true
    }

    pub fn fetch_all(conn: &F5Connection) -> bool {
        // This will actually return an array of F5Virtuals
        true
    }

    pub fn fetch_addrs(conn: &F5Connection) -> bool {
        // This will return an array of IP/ports
        true
    }

    pub fn to_yaml() {
    }

    fn snatpool(conn: &F5Connection) -> bool {
        // This will return a snatpool hashmap
        true
    }

    fn define() {
        // This may not be needed in rust?
    }

    fn validate_rest_call(&self, response: &str, action: &str, conn: &F5Connection) -> bool {
        true
    }

    // No need to implement equality, deriving eq, partialeq does that for us.
}

