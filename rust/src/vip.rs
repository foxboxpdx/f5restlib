use connection::F5Connection;
use types::F5Virtual;
use types::ProfilesReference;
use types::Snat;

impl F5Virtual {
    pub fn new(n: &str, pt: &str, d: &str, ds: &str, pl: &str, sat: Snat, pf: ProfilesReference) -> F5Virtual {
        F5Virtual { name: n.to_string(),
                    partition: pt.to_string(),
                    destination: d.to_string(),
                    description: Some(ds.to_string()),
                    pool: Some(pl.to_string()),
                    snat: sat,
                    profiles: pf }
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

    fn define() {
        // This may not be needed in rust?
    }

    fn validate_rest_call(&self, response: &str, action: &str, conn: &F5Connection) -> bool {
        true
    }

    // No need to implement equality, deriving eq, partialeq does that for us.
}

