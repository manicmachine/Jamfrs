mod api_endpoints;
mod api_token;
mod args;
mod session;

use crate::api_endpoints::ApiEndpoints;
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
    let (_, auth_endpoint) = ApiEndpoints::TokenAuth(&jps_session.server_address).usage();
    let res = client
        .post(auth_endpoint)
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

    let (http_method, api_endpoint) =
        ApiEndpoints::get_endpoint(&args.entity_type, &jps_session.server_address).usage();

    // TODO: Clean this up
    let res = if http_method == reqwest::Method::GET {
        client
            .get(api_endpoint)
            .bearer_auth(&jps_session.api_token.unwrap().token)
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
            .text()
            .unwrap()
    } else if http_method == reqwest::Method::DELETE {
        client
            .delete(api_endpoint)
            .bearer_auth(&jps_session.api_token.unwrap().token)
            .send()
            .unwrap()
            .text()
            .unwrap()
    } else {
        panic!("Invalid HTTP Method encountered");
    };

    // Print results to stdout
    if !args.json && args.pretty {
        let parsed_xml = Element::parse(res.as_bytes()).unwrap();
        let mut emitter_config = EmitterConfig::new();
        emitter_config.perform_indent = true;

        parsed_xml
            .write_with_config(stdout(), emitter_config)
            .unwrap();
    } else if args.json && args.pretty {
        let json_obj: Value = serde_json::from_str(&res).unwrap();
        println!("{}", serde_json::to_string_pretty(&json_obj).unwrap());
    } else {
        println!("{}", res);
    }
}
