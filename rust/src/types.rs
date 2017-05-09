use std::collections::HashMap;

// Virtual
#[derive(Serialize, Deserialize, Debug)]
pub struct VirtualRoot {
    pub kind: String,
    pub items: Vec<F5Virtual>
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct F5Virtual {
    pub name: String,
    pub partition: String,
    pub destination: String,
    pub description: Option<String>,
    pub pool: Option<String>,
    #[serde(rename="sourceAddressTranslation")]
    pub snat: Snat,
    #[serde(rename(serialize="profiles", deserialize="profilesReference"))]
    pub profiles: ProfilesReference
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct ProfilesReference {
    pub items: Vec<VipProfile>
}
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct VipProfile {
    pub name: String,
    pub partition: String,
    pub context: String
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Snat {
    pub pool: String,
    #[serde(rename="type")]
    pub stype: String
}

// Pool
#[derive(Serialize, Deserialize, Debug)]
pub struct PoolRoot {
    pub kind: String,
    pub items: Vec<F5Pool>
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct F5Pool {
    pub name: String,
    pub partition: String,
    pub description: Option<String>,
    #[serde(rename="loadBalancingMode")]
    pub lbmode: String,
    pub monitor: String,
    #[serde(rename(serialize="members", deserialize="membersReference"))]
    pub members: MembersReference
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct MembersReference {
    pub items: Vec<PoolMember>
}
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct PoolMember {
    pub name: String,
    pub partition: String
}

// ClientSSLProfile
#[derive(Serialize, Deserialize, Debug)]
pub struct ClientRoot {
    pub kind: String,
    pub items: Vec<F5ClientSSL>
}
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct F5ClientSSL {
    pub name: String,
    pub partition: String,
    pub cert: String,
    pub key: String,
    #[serde(rename="defaultsFrom")]
    pub defaults: Option<String>,
    pub description: Option<String>
}

// Cert
#[derive(Serialize, Deserialize, Debug)]
pub struct CertRoot {
    pub kind: String,
    pub items: Vec<F5Cert>
}
#[derive(Serialize, Deserialize, Debug)]
pub struct F5Cert {
    pub name: String,
    #[serde(rename="apiRawValues")]
    pub raw: HashMap<String, String>,
    #[serde(rename="commonName")]
    pub cn: String
}

// Key
#[derive(Deserialize, Debug)]
pub struct KeyRoot {
    pub kind: String,
    pub items: Vec<F5Key>
}
#[derive(Deserialize, Debug)]
pub struct F5Key {
    pub name: String,
    #[serde(rename="keySize")]
    pub key_size: String,
    #[serde(rename="keyType")]
    pub key_type: String
}

// Device-groups
#[derive(Deserialize, Debug)]
pub struct DeviceGroupRoot {
    pub kind: String,
    pub items: Vec<F5DeviceGroup>
}
#[derive(Deserialize, Debug)]
pub struct F5DeviceGroup {
    pub name: String,
    #[serde(rename="type")]
    pub dgtype: String
}
