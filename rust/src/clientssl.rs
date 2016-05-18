use connection::F5Connection;
use types::F5ClientSSL;

impl F5ClientSSL {
    pub fn new(n: &str, p: &str, c: &str, k: &str) -> F5ClientSSL {
        F5ClientSSL { name: n.to_string(),
                      partition: p.to_string(),
                      cert: c.to_string(),
                      key: k.to_string() }
    }
}

