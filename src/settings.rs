use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Settings {
    pub server: Server,
    pub keys: Keys,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Keys {
    pub ca_private_key_path: String,
    pub ca_root_certificate_path: String,
}

impl Default for Keys {
    fn default() -> Keys {
        Keys {
            ca_root_certificate_path: "./certs/ca.cert".to_owned(),
            ca_private_key_path: "./keys/ca.private".to_owned(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Server {
    pub host: String,
}

impl Default for Server {
    fn default() -> Server {
        Server {
            host: "127.0.0.1:5656".to_owned(),
        }
    }
}
