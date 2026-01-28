use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub db_path: String,
    pub addr: String,
    pub pub_addr: String,
}

impl Config {
    pub fn new(db_path: String, addr: String, pub_addr: String) -> Self {
        Self {
            db_path,
            addr,
            pub_addr,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            db_path: "database.sqlite".to_owned(),
            addr: "0.0.0.0:8080".to_owned(),
            pub_addr: "mail.example.com:8080".to_owned(),
        }
    }
}
