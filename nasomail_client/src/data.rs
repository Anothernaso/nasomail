use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct AppData {
    server_addr: String,
}

impl AppData {
    pub fn new(server_addr: String) -> Self {
        Self { server_addr }
    }

    pub fn server_addr(&self) -> &str {
        &self.server_addr
    }
    pub fn server_addr_mut(&mut self) -> &mut str {
        &mut self.server_addr
    }
    pub fn set_server_addr(&mut self, value: String) {
        self.server_addr = value;
    }
}
