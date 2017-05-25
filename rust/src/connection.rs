extern crate serde;
extern crate serde_json;
//use hyper::Url;
use hyper::Client;
use hyper::client::Response;
use hyper::header::*;
use hyper::mime::Mime;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::collections::HashMap;
use serde_json::Value;
use types::DeviceGroupRoot;
//use std::io::Error;

#[derive(Debug)]
pub struct F5Connection {
    username: String,
    password: String,
    hostname: String,
    basename: String
}

impl F5Connection {
    pub fn new(u: &str, p: &str, h: &str) -> F5Connection {
        let b = str::replace(h, "https://", "");
        F5Connection { username: u.to_string(),
                       password: p.to_string(),
                       hostname: h.to_string(),
                       basename: b.to_string() }
    }

    pub fn check_connection(&self) -> bool {
        let client = Client::new();
        let uri = self.hostname.clone() + "/mgmt/tm/cm/device/" + self.basename.as_str();
        let headers = self.set_headers();
        let mut res = client.get(&uri).headers(headers).send().unwrap();
        if res.status.is_success() {
            let mut body = String::new();
            res.read_to_string(&mut body).expect("Error reading response");
            let parsed: Value = serde_json::from_str(body.as_str()).expect("Can't parse response");
            if parsed["failoverState"] == "standby" {
                println!("{} is not the active F5 device", self.basename);
                return false
            }
        } else {
            println!("Error retrieving device status");
            return false
        }
        true
    }

    pub fn sync(&self) -> bool {
        let client = Client::new();
        // First query will get the name of the sync-failover group
        let mut group = String::new();
        let mut uri = self.hostname.clone() + "/mgmt/tm/cm/device-group/";
        let headers = self.set_headers();
        let mut res = client.get(&uri).headers(headers).send().unwrap();
        if !res.status.is_success() {
            println!("Error retrieving device-group data");
            return false
        }
        let mut body = String::new();
        res.read_to_string(&mut body).expect("Error reading response");
        let dgr: DeviceGroupRoot = serde_json::from_str(&body).expect("Error converting JSON");
        // Iterate through the DGs looking for type=sync-failover
        for g in &dgr.items {
            if g.dgtype == "sync-failover" {
                group = g.name.clone();
                break;
            }
        }
        if group.is_empty() {
            println!("Could not find a sync-failover group");
            return false
        }
        // Now call the sync operation on this group
        let cmd = format!("config-sync to-group {}", group);
        let mut payload = HashMap::new();
        payload.insert("command", "run");
        payload.insert("utilCmdArgs", &cmd);
        let outgoing = serde_json::to_string(&payload).unwrap(); 
        uri = self.hostname.clone() + "/mgmt/tm/cm/";
        let headers = self.set_headers();
        match client.post(&uri).body(&outgoing).headers(headers).send() {
            Ok(_) => {
                println!("Called sync ok");
                return true
            },
            Err(e) => {
                println!("Error calling sync: {}", e);
                return false
            }
        }
    }

    pub fn get(&self, uri: &str) -> Response {
        let client = Client::new();
        let full_uri = self.hostname.clone() + uri + "?expandSubcollections=true";
        let headers = self.set_headers();
        /* Make the caller responsible for matching(ok/err) here */
        client.get(&full_uri).headers(headers).send().unwrap()
    }

    pub fn post<'a, T>(&self, uri: &str, payload: &T) -> Response where T: serde::Serialize { 
        let body = serde_json::to_string(&payload).unwrap();
        let client = Client::new();
        let full_uri = self.hostname.clone() + uri;
        let headers = self.set_headers();
        let res = client.post(&full_uri).body(&body).headers(headers).send().unwrap();
        res
    }

    pub fn put<'a, T>(&self, uri: &str, payload: &T) -> Response where T: serde::Serialize {
        let body = serde_json::to_string(&payload).unwrap();
        let client = Client::new();
        let full_uri = self.hostname.clone() + uri;
        let headers = self.set_headers();
        let res = client.post(&full_uri).body(&body).headers(headers).send().unwrap();
        res
    }

    pub fn delete(&self, uri: &str) -> Response {
        let client = Client::new();
        let full_uri = self.hostname.clone() + uri;
        let headers = self.set_headers();
        let res = client.delete(&full_uri).headers(headers).send().unwrap();
        res
    }

    pub fn upload_file(&self, filename: &str) -> bool {
        let mut payload = HashMap::new();
        payload.insert("command", "run");
        payload.insert("utilCmdArgs","-c 'mkdir /var/config/rest/downloads/tmp'");
        //let mut body: String = json::encode(&payload).unwrap();
        let body = serde_json::to_string(&payload).unwrap();
        let client = Client::new();
        let mut full_uri: String = self.hostname.clone() + "/mgmt/tm/util/bash";
        let headers = self.set_headers();
        match client.post(&full_uri).body(&body).headers(headers).send() {
            Ok(_) => {},
            Err(e)  => {
                println!("Error making upload dir: {}", e);
                return false
            }
        }
        
        let path = Path::new(filename);
        let file = path.file_name().unwrap().to_str().unwrap(); // *cry*
        println!("Uploading {} to /var/config/rest/downloads/tmp/{}", file, file);
        let mut f = File::open(filename).unwrap();
        let mut buffer = String::new();
        f.read_to_string(&mut buffer).unwrap();
        let ohead = self.octet_headers(buffer.len() as u64);
        full_uri.clear();
        full_uri = self.hostname.clone() + "/mgmt/shared/file-transfer/uploads/" + file;
        match client.post(&full_uri).body(&buffer).headers(ohead).send() {
            Ok(_) => {},
            Err(e) => {
                println!("Error uploading cert file: {}", e);
                return false
            }
        }
        true
    }

    pub fn set_headers(&self) -> Headers {
        let mut headers = Headers::new();
        headers.set(
            Authorization(
                Basic {
                    username: self.username.clone(),
                    password: Some(self.password.clone())
                }
            ),
        );
        headers.set(ContentType::json());
        headers
    }

    pub fn octet_headers(&self, length: u64) -> Headers {
        let mut headers = Headers::new();
        headers.set(
            Authorization(
                Basic {
                    username: self.username.clone(),
                    password: Some(self.password.clone())
                }
            ),
        );
        let aos: Mime = "application/octet-stream".parse().unwrap();
        headers.set(ContentType(aos));
        let lenminusone = (length - 1).to_string();
        let fulllength = length.to_string();
        let rangestring = "0-".to_string() + &lenminusone + "/" + &fulllength;
        headers.set(ContentRange::parse_header(&[rangestring.into_bytes()]).unwrap());
        headers
    }
}
