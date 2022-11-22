mod api_endpoints;
mod api_token;
mod args;
mod session;

use crate::api_endpoints::{ApiDetails, ApiEndpoints, Args};
use crate::api_token::ApiToken;
use crate::session::Session;
use args::JamfrsArgs;
use clap::Parser;
use reqwest::blocking::Client;
use serde_json::Value;
use std::io::stdout;
use std::process::exit;
use xmltree::{Element, EmitterConfig};

fn main() {
    let args = JamfrsArgs::parse();
    let client = Client::builder()
        .danger_accept_invalid_certs(args.insecure)
        .build()
        .unwrap();
    let mut jps_session = match Session::new(
        args.server_address.clone(),
        args.port,
        args.username.clone(),
        args.password.clone(),
        args.insecure,
    ) {
        Ok(session) => session,
        Err(err) => {
            eprintln!("{}", err);
            exit(1);
        }
    };

    // Authenticate with the server and store the token
    let api_details = ApiEndpoints::TokenAuth.usage();
    let res = client
        .post(format!(
            "{}{}",
            &jps_session.server_address, api_details.url
        ))
        .basic_auth(&jps_session.username, Some(&jps_session.password));

    jps_session.api_token = match res.send() {
        Ok(res) => {
            if res.status().is_success() {
                Some(res.json::<ApiToken>().unwrap())
            } else {
                eprintln!(
                    "Failed to retrieve auth token from {}. {}",
                    jps_session.server_address,
                    res.status()
                );
                None
            }
        }
        Err(err) => {
            eprintln!(
                "Failed to retrieve auth token from {}: {}",
                jps_session.server_address, err
            );
            None
        }
    };

    if jps_session.api_token.is_none() {
        exit(1);
    }

    let api_details = ApiEndpoints::get_api_details(&args.entity_type);
    let mut url_builder = UrlBuilder::new(&jps_session.server_address, api_details);

    // TODO: Clean this up
    loop {
        let res = if url_builder.api_details.endpoint.method == reqwest::Method::GET {
            client
                .get(url_builder.build_next_url())
                .bearer_auth(&jps_session.api_token.as_ref().unwrap().token)
                .header(
                    "accept",
                    if args.json {
                        "application/json"
                    } else {
                        "application/xml"
                    },
                )
                .send()
                .unwrap()
        } else if url_builder.api_details.endpoint.method == reqwest::Method::DELETE {
            client
                .delete(url_builder.build_next_url())
                .bearer_auth(&jps_session.api_token.as_ref().unwrap().token)
                .send()
                .unwrap()
        } else {
            panic!("Invalid HTTP Method encountered");
        };

        // TODO: Improve error reporting
        if !res.status().is_success() {
            eprintln!("Error: Server returned 404 for request: {}", res.url().path());
        }


        // Print results to stdout
        if !args.json && args.pretty {
            let parsed_xml = Element::parse(res.text().unwrap().as_bytes()).unwrap();
            let mut emitter_config = EmitterConfig::new();
            emitter_config.perform_indent = true;

            parsed_xml
                .write_with_config(stdout(), emitter_config)
                .unwrap();
        } else if args.json && args.pretty {
            let json_obj: Value = serde_json::from_str(&res.text().unwrap()).unwrap();
            println!("{}", serde_json::to_string_pretty(&json_obj).unwrap());
        } else {
            println!("{}", res.text().unwrap());
        }

        if url_builder.done { break }
    }
}

// TODO: Extract this into it's own module
struct UrlBuilder<'a> {
    address: &'a String,
    pub api_details: ApiDetails<'a>,
    arg_index: usize,
    done: bool
}

impl<'a> UrlBuilder<'a> {
    fn new(address: &'a String, api_details: ApiDetails<'a>) -> Self {
        UrlBuilder {
            address,
            api_details,
            arg_index: 0,
            done: false
        }
    }

    fn build_next_url(&mut self) -> String {
        match self.api_details.args {
            Args::None => {
                self.done = true;
                format!("{}{}", self.address, self.api_details.endpoint.url)
            }
            Args::String(string) => {
                self.done = true;
                format!("{}{}", self.address, self.api_details.endpoint.url).replace("{val}", string)
            }
            Args::Ids(ids) => {
                let url = format!("{}{}", self.address, self.api_details.endpoint.url).replace("{val}", ids.get(self.arg_index).unwrap().to_string().as_str());
                self.arg_index += 1;

                if self.arg_index == ids.len() {
                    self.done = true;
                }

                url
            }
        }
    }
}