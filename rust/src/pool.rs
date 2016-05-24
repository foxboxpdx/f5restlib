use connection::F5Connection;
use types::F5Pool;
use types::PoolMember;
use types::Members;

impl F5Pool {
    pub fn new(n: &str, p: &str, lbm: &str, mon: &str, mem: Members) -> F5Pool {
        F5Pool { name: n.to_string(),
                 partition: p.to_string(),
                 lbmode: lbm.to_string(),
                 monitor: mon.to_string(),
                 members: mem }
    }

    pub fn from_yaml(data: &str) -> bool {
        // This should actually return an array of F5Pools
        true
    }

    pub fn create(&self, conn: &F5Connection) -> bool {
        true
    }

    pub fn delete(&self, conn: &F5Connection) -> bool {
        true
    }

    pub fn delta(&self, conn: &F5Connection) -> bool {
        true
    }

    pub fn fetch(conn: &F5Connection, partition: &str, name: &str) -> bool {
        // Will return an F5Pool
        true
    }

    pub fn fetch_all(conn: &F5Connection) -> bool {
        // Will return array of F5Pools
        true
    }

    pub fn to_yaml() {
    }

    fn define() {
        // Is this needed?
    }

    fn validate_rest_call(&self, response: &str, action: &str, conn: &F5Connection) -> bool {
        true
    }

}

impl PoolMember {
    pub fn new(n: &str) -> PoolMember {
        PoolMember { name: n.to_string() }
    }
}

impl Members {
    pub fn new(i: Vec<PoolMember>) -> Members {
        Members { items: i }
    }
}

