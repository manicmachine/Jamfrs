pub mod api_service {
    pub mod api_endpoints;
    mod session;
    use api_endpoints::{ApiDetails, ApiEndpoints, Args};
    use reqwest::Client;
    use reqwest::Method;
    use session::Session;
    use std::time::{SystemTime, UNIX_EPOCH};
    use tokio::sync::mpsc::{channel, Receiver};

    pub struct JamfApiService {
        client: Client,
        jps_session: Session,
        json: bool,
        url_builder: Option<UrlBuilder>,
    }

    // TODO: Return different Error types to better identify the cause of an error so that we can make better decisions in response
    impl JamfApiService {
        pub fn new(
            server_address: String,
            port: Option<u16>,
            username: String,
            password: String,
            insecure: bool,
            json: bool,
        ) -> Result<Self, String> {
            let client = match Client::builder()
                .danger_accept_invalid_certs(insecure)
                .build()
            {
                Ok(client) => client,
                Err(err) => return Err(err.to_string()),
            };

            let jps_session = match Session::new(server_address, port, username, password, insecure)
            {
                Ok(session) => session,
                Err(err) => return Err(err.to_string()),
            };

            Ok(Self {
                client,
                jps_session,
                json,
                url_builder: None,
            })
        }

        pub fn set_commands(&mut self, commands: ApiDetails) -> Result<&ApiDetails, String> {
            self.url_builder = Some(UrlBuilder::new(
                self.jps_session.server_address.clone(),
                commands,
            ));

            let api_details_ref = &self.url_builder.as_ref().unwrap().api_details;
            Ok(api_details_ref)
        }

        pub fn number_of_commands(&self) -> u32 {
            match self.url_builder {
                None => 0,
                Some(ref url_builder) => match &url_builder.api_details.args {
                    Args::None => 1,
                    Args::Strings(_) => 1,
                    Args::Ids(ids) => ids.len() as u32,
                },
            }
        }

        pub async fn process_commands(&mut self) -> Receiver<Result<String, String>> {
            // TODO: Reimplement such that tasks can re-authenticate as necessary
            if !self.token_is_valid() {
                self.authenticate()
                    .await
                    .expect("Failed to authenticate with server");
            }

            let accept_type = format!("application/{}", if self.json { "json" } else { "xml" });
            let (tx, rx) = channel(self.number_of_commands() as usize);

            while let Some(url) = self.url_builder.as_mut().unwrap().next() {
                let mut res_builder = match self
                    .url_builder
                    .as_ref()
                    .unwrap()
                    .api_details
                    .endpoint
                    .method
                {
                    Method::GET => self.client.get(url),
                    Method::POST => self.client.post(url),
                    Method::PUT => self.client.put(url),
                    Method::DELETE => self.client.delete(url),
                    _ => panic!("Invalid HTTP method provided"),
                };

                res_builder = res_builder
                    .bearer_auth(&self.jps_session.api_token.as_ref().unwrap().token)
                    .header("accept", &accept_type);

                let tx_clone = tx.clone();
                tokio::spawn(async move {
                    match res_builder.send().await {
                        Ok(res) => {
                            if res.status().is_success() {
                                tx_clone.send(Ok(res.text().await.unwrap())).await
                            } else {
                                tx_clone
                                    .send(Err(format!("{} for {}", res.status(), res.url().path())))
                                    .await
                            }
                        }
                        Err(err) => tx_clone.send(Err(err.to_string())).await,
                    }
                });
            }

            rx
        }

        async fn authenticate(&mut self) -> Result<(), String> {
            let res = self
                .client
                .post(format!(
                    "{}{}",
                    &self.jps_session.server_address,
                    ApiEndpoints::TokenAuth.usage().url
                ))
                .basic_auth(&self.jps_session.username, Some(&self.jps_session.password))
                .send()
                .await;

            match res {
                Ok(token_res) => {
                    if token_res.status().is_success() {
                        self.jps_session
                            .create_auth_token(token_res.text().await.unwrap())
                    } else {
                        Err(token_res.status().to_string())
                    }
                }
                Err(err) => Err(err.to_string()),
            }
        }

        fn token_is_valid(&self) -> bool {
            return if self.jps_session.api_token.is_some() {
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
                    .le(&self.jps_session.api_token.as_ref().unwrap().expires)
            } else {
                false
            };
        }
    }

    struct UrlBuilder {
        address: String,
        api_details: ApiDetails,
        arg_index: usize,
    }

    impl UrlBuilder {
        fn new(address: String, api_details: ApiDetails) -> Self {
            UrlBuilder {
                address,
                api_details,
                arg_index: 0,
            }
        }
    }

    impl Iterator for UrlBuilder {
        type Item = String;

        fn next(&mut self) -> Option<Self::Item> {
            match &self.api_details.args {
                Args::None => {
                    if self.arg_index == 0 {
                        self.arg_index += 1;
                        Some(format!("{}{}", self.address, self.api_details.endpoint.url))
                    } else {
                        None
                    }
                }
                Args::Strings(args) => {
                    if self.arg_index == 0 {
                        self.arg_index += 1;
                        let mut url = format!("{}{}", self.address, self.api_details.endpoint.url);
                        for (placeholder, arg) in args {
                            url = url.replace(placeholder, arg);
                        }

                        Some(url)
                    } else {
                        None
                    }
                }
                Args::Ids(ids) => {
                    if self.arg_index < ids.len() {
                        let url = format!("{}{}", self.address, self.api_details.endpoint.url)
                            .replace("{id}", ids.get(self.arg_index).unwrap());
                        self.arg_index += 1;
                        Some(url)
                    } else {
                        None
                    }
                }
            }
        }
    }
}
