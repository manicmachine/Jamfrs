use crate::api_token::ApiToken;

#[derive(Debug)]
pub struct Session {
    pub server_address: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub api_token: Option<ApiToken>,
}

impl Session {
    pub fn new(
        server_address: String,
        port: Option<u16>,
        username: String,
        password: String,
    ) -> Self {
        Self {
            server_address: server_address.to_lowercase().trim().to_string(),
            username: username.trim().to_string(),
            password: password.trim().to_string(),
            api_token: None,
            port: match port {
                Some(port) => port,
                None => {
                    if server_address.to_lowercase().contains("jamfcloud.com") {
                        443
                    } else {
                        8443
                    }
                }
            },
        }
    }
}
