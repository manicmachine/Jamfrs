use crate::api_token::ApiToken;
use regex::Regex;
use Result::Err;

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
        insecure: bool,
    ) -> Result<Self, &'static str> {
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
                    let mut port_str =
                        port_pattern.captures(&server_address).unwrap()[0].to_string();
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
                addr.push_str(if insecure { "http://" } else { "https://" });
            }

            addr.push_str(server_address.as_str());

            // Check if the user provided the port number as part of the server_address, and if so,
            // do not append the port provided by the --port flag to the address
            if !port_pattern.is_match(&addr) {
                addr.push_str(format!(":{port_val}").as_str());
            } else if port.is_some() {
                return Err("Error: Don't include a port with the server_address while using the --port flag");
            }

            addr
        };

        Ok(Self {
            server_address: server_address_val,
            port: port_val,
            username,
            password,
            insecure,
            api_token: None,
        })
    }

    pub fn create_auth_token(&mut self, token_string: String) -> Result<(), String> {
        self.api_token = match serde_json::from_str(token_string.as_str()) {
            Ok(token) => Some(token),
            Err(err) => {
                return Err(err.to_string());
            }
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::Session;

    #[test]
    fn test_jamfcloud() {
        let session = Session::new(
            String::from("test.jamfcloud.com"),
            None,
            String::from("test"),
            String::from("test-password"),
            false,
        )
        .unwrap();

        assert!(session.server_address.starts_with("https://"));
        assert_eq!(session.port, 443);
    }

    #[test]
    fn test_jps_default_port() {
        // Hostname
        let session = Session::new(
            String::from("test.jps.com"),
            None,
            String::from("test"),
            String::from("test-password"),
            false,
        )
        .unwrap();

        assert!(session.server_address.starts_with("https://"));
        assert_eq!(session.port, 8443);

        let session = Session::new(
            String::from("192.168.1.1"),
            None,
            String::from("test"),
            String::from("test-password"),
            false,
        )
        .unwrap();

        assert!(session.server_address.starts_with("https://"));
        assert_eq!(session.port, 8443);
    }

    #[test]
    fn test_jps_specified_port() {
        let session = Session::new(
            String::from("test.jps.com"),
            Some(2022),
            String::from("test"),
            String::from("test-password"),
            false,
        )
        .unwrap();

        assert!(session.server_address.starts_with("https://"));
        assert_eq!(session.port, 2022);

        let session = Session::new(
            String::from("192.168.1.1"),
            Some(2022),
            String::from("test"),
            String::from("test-password"),
            false,
        )
        .unwrap();

        assert!(session.server_address.starts_with("https://"));
        assert_eq!(session.port, 2022);
    }

    #[test]
    fn test_jps_insecure_default_port() {
        let session = Session::new(
            String::from("test.jps.com"),
            None,
            String::from("test"),
            String::from("test-password"),
            true,
        )
        .unwrap();

        assert!(session.server_address.starts_with("http://"));
        assert_eq!(session.port, 8080);

        let session = Session::new(
            String::from("192.168.1.1"),
            None,
            String::from("test"),
            String::from("test-password"),
            true,
        )
        .unwrap();

        assert!(session.server_address.starts_with("http://"));
        assert_eq!(session.port, 8080);
    }

    #[test]
    fn test_jps_inline_port() {
        let session = Session::new(
            String::from("test.jps.com:2022"),
            None,
            String::from("test"),
            String::from("test-password"),
            false,
        )
        .unwrap();

        assert!(session.server_address.starts_with("https://"));
        assert_eq!(session.port, 2022);

        let session = Session::new(
            String::from("192.168.1.1:2022"),
            None,
            String::from("test"),
            String::from("test-password"),
            false,
        )
        .unwrap();

        assert!(session.server_address.starts_with("https://"));
        assert_eq!(session.port, 2022);
    }

    #[test]
    fn test_jps_error_when_both_inline_and_port_flag_used() {
        let session = Session::new(
            String::from("test.jps.com:2022"),
            Some(2022),
            String::from("test"),
            String::from("test-password"),
            false,
        );

        assert!(session.is_err());
    }

    #[test]
    fn test_jps_inline_proto() {
        let session = Session::new(
            String::from("https://test.jps.com"),
            None,
            String::from("test"),
            String::from("test-password"),
            false,
        )
        .unwrap();

        assert!(session.server_address.starts_with("https://"));

        let session = Session::new(
            String::from("http://test.jps.com"),
            None,
            String::from("test"),
            String::from("test-password"),
            true,
        )
        .unwrap();

        assert!(session.server_address.starts_with("http://"));
    }
}
