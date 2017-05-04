use connection::F5Connection;
use types::F5Pool;
use types::PoolMember;
use types::MembersReference;

impl F5Pool {
    pub fn new(n: &str, p: &str, d: &str, lbm: &str, mon: &str, mem: MembersReference) -> F5Pool {
        F5Pool { name: n.to_string(),
                 partition: p.to_string(),
                 description: Some(d.to_string()),
                 lbmode: lbm.to_string(),
                 monitor: mon.to_string(),
                 members: mem }
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

    fn define() {
        // Is this needed?
    }

    fn validate_rest_call(&self, response: &str, action: &str, conn: &F5Connection) -> bool {
        true
    }

}

impl PoolMember {
    pub fn new(n: &str, p: &str) -> PoolMember {
        PoolMember { name: n.to_string(),
                     partition: p.to_string() }
    }
}

impl MembersReference {
    pub fn new(i: Vec<PoolMember>) -> MembersReference {
        MembersReference { items: i }
    }
}

