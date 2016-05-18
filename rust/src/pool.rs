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

