use std::process::exit;
use regex::Regex;
use crate::api_token::ApiToken;

#[derive(Debug)]
pub struct Session {
    pub server_address: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub insecure: bool,
    pub api_token: Option<ApiToken>,
}

impl Session {
    pub fn new(
        server_address: String,
        port: Option<u16>,
        username: String,
        password: String,
        insecure: bool
    ) -> Self {
        let proto_pattern = Regex::new(r"^https?://").unwrap();
        let port_pattern = Regex::new(r":[0-9]*$").unwrap();

        let port_val = match port {
            Some(port) => port,
            None => {
                if server_address.to_lowercase().contains("jamfcloud.com") {
                    // jamfcloud doesn't support non-secure communications, so only port 443 is valid
                    443
                } else if port_pattern.is_match(&server_address) {
                    // Extract the port value from the server_address
                    let mut port_str = port_pattern.captures(&server_address).unwrap()[0].to_string();
                    port_str.remove(0); // remove semi-colon

                    port_str.parse::<u16>().unwrap()
                } else if insecure {
                    8080
                } else {
                    8443
                }
            }
        };

        let server_address_val = {
            let mut addr = String::new();

            // Check if the user provided a protocol as part of the server_address, and if so,
            // do not prepend the protocol implied by the --insecure flag
            if !proto_pattern.is_match(&server_address) {
                addr.push_str(if insecure { "http://" } else { "https://"});
            }

            addr.push_str(server_address.as_str());

            // Check if the user provided the port number as part of the server_address, and if so,
            // do not append the port provided by the --port flag to the address
            if !port_pattern.is_match(&addr) {
                addr.push_str(format!(":{}", port_val).as_str());
            } else if port.is_some() {
                eprintln!("Error: Don't include a port with the server_address while using the --port flag");
                exit(1);
            }

            addr
        };

        Self {
            server_address: server_address_val,
            port: port_val,
            username,
            password,
            insecure,
            api_token: None
        }
    }
}
