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
}

