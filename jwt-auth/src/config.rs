pub struct AppConfig {
    pub private_pem: String,
    pub public_pem: String,
}

impl AppConfig {
    pub fn new() -> Self {
        Self {
            private_pem: include_str!("../keys/private.pem").to_string(),
            public_pem: include_str!("../keys/public.pem").to_string(),
        }
    }
}
