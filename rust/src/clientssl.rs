use connection::F5Connection;
use types::F5ClientSSL;

impl F5ClientSSL {
    pub fn new(n: &str, p: &str, d: &str, c: &str, k: &str, df: &str) -> F5ClientSSL {
        F5ClientSSL { name: n.to_string(),
                      partition: p.to_string(),
                      description: Some(d.to_string()),
                      cert: c.to_string(),
                      key: k.to_string(),
                      defaults: Some(df.to_string()) }
    }

    pub fn from_yaml(data: &str) -> bool {
        true
    }

    pub fn create(&self, conn: &F5Connection) -> bool {
        true
    }

    pub fn delete(&self, conn: &F5Connection) -> bool {
        true
    }

    pub fn fetch(conn: &F5Connection, partition: &str, name: &str) -> bool {
        true
    }

    pub fn fetch_all(conn: &F5Connection) -> bool {
        true
    }

    pub fn to_yaml() {
    }

    fn define() {
    }

    fn validate_rest_call(&self, response: &str, action: &str, conn: &F5Connection) -> bool {
        true
    }

}

