use connection::F5Connection;

#[derive(Debug)]
pub struct F5PKCS12 {
    pub name: String,
    pub partition: String,
    pub filename: String,
    pub password: String,
    pub uploaded: bool
}

impl F5PKCS12 {
    pub fn new(n: &str, p: &str, f: &str, pw: &str) -> F5PKCS12 {
        F5PKCS12 { name: n.to_string(),
                   partition: p.to_string(),
                   filename: f.to_string(),
                   password: pw.to_string(),
                   uploaded: false }
    }

    pub fn from_yaml(data: &str) -> bool {
        true
    }

    pub fn create(&self, conn: &F5Connection) -> bool {
        true
    }

    pub fn upload(&self, conn: &F5Connection) -> bool {
        true
    }

    fn validate_rest_call(&self, action: &str, conn: &F5Connection) -> bool {
        true
    }
}
