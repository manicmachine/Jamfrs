use crate::args::EntityType;
use crate::{ApiDetails, ApiEndpoints, Args, Session};
use reqwest::Client;
use reqwest::Method;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::mpsc::{channel, Sender, Receiver};
use tokio::task::JoinHandle;

pub struct ApiService<'a> {
    client: Client,
    jps_session: Session,
    json: bool,
    url_builder: Option<UrlBuilder<'a>>,
}

// TODO: Return different Error types to better identify the cause of an error so that we can make better decisions in response
impl<'a> ApiService<'a> {
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

        let jps_session = match Session::new(server_address, port, username, password, insecure) {
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

    pub fn set_commands(&mut self, commands: &'a EntityType) {
        self.url_builder = Some(UrlBuilder::new(
            self.jps_session.server_address.clone(),
            ApiEndpoints::get_api_details(commands),
        ));
    }

    pub fn number_of_commands(&self) -> u32 {
        match self.url_builder {
            None => 0,
            Some(ref url_builder) => match url_builder.api_details.args {
                Args::None => 1,
                Args::String(_) => 1,
                Args::Ids(ids) => ids.len() as u32,
            },
        }
    }

    pub async fn process_commands(&mut self) ->  Receiver<Result<String, String>> {
        // TODO: Reimplement such that tasks can re-authenticate as necessary
        if !self.token_is_valid() {
            self.authenticate().await.expect("Failed to authenticate with server");
        }

        let accept_type = format!("application/{}", if self.json { "json " } else { "xml" });

        let (tx, rx) = channel(self.number_of_commands() as usize);
        let mut i = 0;
        let iterations = self.number_of_commands();

        while i < iterations {
            let mut res_builder = match self
                .url_builder
                .as_ref()
                .unwrap()
                .api_details
                .endpoint
                .method
            {
                Method::GET => self
                    .client
                    .get(self.url_builder.as_mut().unwrap().build_next_url()),
                Method::POST => self
                    .client
                    .post(self.url_builder.as_mut().unwrap().build_next_url()),
                Method::PUT => self
                    .client
                    .put(self.url_builder.as_mut().unwrap().build_next_url()),
                Method::DELETE => self
                    .client
                    .delete(self.url_builder.as_mut().unwrap().build_next_url()),
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
                            tx_clone.send(Err(format!("Status {} received for {}", res.status().is_success(), res.url().path()))).await
                        }
                    }
                    Err(err) => tx_clone.send(Err(err.to_string())).await,
                }
            });

            i += 1;
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

struct UrlBuilder<'a> {
    address: String,
    api_details: ApiDetails<'a>,
    arg_index: usize,
}

impl<'a> UrlBuilder<'a> {
    fn new(address: String, api_details: ApiDetails<'a>) -> Self {
        UrlBuilder {
            address,
            api_details,
            arg_index: 0,
        }
    }

    fn build_next_url(&mut self) -> String {
        match self.api_details.args {
            Args::None => {
                format!("{}{}", self.address, self.api_details.endpoint.url)
            }
            Args::String(string) => format!("{}{}", self.address, self.api_details.endpoint.url)
                .replace("{val}", string),
            Args::Ids(ids) => {
                let url = format!("{}{}", self.address, self.api_details.endpoint.url).replace(
                    "{val}",
                    ids.get(self.arg_index).unwrap().to_string().as_str(),
                );
                self.arg_index += 1;

                url
            }
        }
    }
}
