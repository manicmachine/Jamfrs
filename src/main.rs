mod api_endpoints;
mod api_token;
mod args;
mod session;

use crate::api_endpoints::ApiEndpoints;
use crate::api_token::ApiToken;
use crate::args::{ComputerSubcommand, EntityType, MobileSubcommand, UserSubcommand};
use crate::session::Session;
use args::JamfrsArgs;
use clap::Parser;
use reqwest::blocking::Client;
use serde_json::Value;
use std::io::stdout;
use xmltree::{Element, EmitterConfig};

fn main() {
    let client = Client::new();
    let args = JamfrsArgs::parse();
    let mut jps_session =
        Session::new(args.server_address, args.port, args.username, args.password);

    // TODO: Has to be a better way; clean this up
    // Check if user entered a protocol, and if so, remove it
    jps_session.server_address = if jps_session.server_address.starts_with("http://") {
        jps_session.server_address[7..].to_string()
    } else if jps_session.server_address.starts_with("https://") {
        jps_session.server_address[8..].to_string()
    } else {
        jps_session.server_address
    };

    // Authenticate with the server and store the token
    let res = client
        .post(ApiEndpoints::TokenAuth(&jps_session.server_address).value())
        .basic_auth(&jps_session.username, Some(&jps_session.password));

    jps_session.api_token = match res.send() {
        Ok(res) => Some(res.json::<ApiToken>().unwrap()),
        Err(err) => panic!(
            "Error getting auth token from {}: {err}",
            jps_session.server_address
        ),
    };

    // Determine HTTP method and api endpoint for given command
    let (http_method, api_endpoint) = match args.entity_type {
        EntityType::Computer(command) => match command.command {
            ComputerSubcommand::Delete { id } => (
                reqwest::Method::DELETE,
                ApiEndpoints::ComputerDelete {
                    host: &jps_session.server_address,
                    id,
                },
            ),
            ComputerSubcommand::Show { id } => (
                reqwest::Method::GET,
                ApiEndpoints::ComputerShow {
                    host: &jps_session.server_address,
                    id,
                },
            ),
            ComputerSubcommand::Search { search_query } => (
                reqwest::Method::GET,
                ApiEndpoints::ComputerSearch {
                    host: &jps_session.server_address,
                    query_string: search_query,
                },
            ),
            ComputerSubcommand::List => (
                reqwest::Method::GET,
                ApiEndpoints::ComputerList(&jps_session.server_address),
            ),
        },
        EntityType::Mobile(command) => match command.command {
            MobileSubcommand::Delete { id } => (
                reqwest::Method::DELETE,
                ApiEndpoints::MobileDelete {
                    host: &jps_session.server_address,
                    id,
                },
            ),
            MobileSubcommand::Show { id } => (
                reqwest::Method::GET,
                ApiEndpoints::MobileShow {
                    host: &jps_session.server_address,
                    id,
                },
            ),
            MobileSubcommand::Search { search_query } => (
                reqwest::Method::GET,
                ApiEndpoints::MobileSearch {
                    host: &jps_session.server_address,
                    query_string: search_query,
                },
            ),
            MobileSubcommand::List => (
                reqwest::Method::GET,
                ApiEndpoints::MobileList(&jps_session.server_address),
            ),
        },
        EntityType::User(command) => match command.command {
            UserSubcommand::Delete { id } => (
                reqwest::Method::DELETE,
                ApiEndpoints::UserDelete {
                    host: &jps_session.server_address,
                    id,
                },
            ),
            UserSubcommand::Show { id } => (
                reqwest::Method::GET,
                ApiEndpoints::UserShow {
                    host: &jps_session.server_address,
                    id,
                },
            ),
            UserSubcommand::List => (
                reqwest::Method::GET,
                ApiEndpoints::UserList(&jps_session.server_address),
            ),
        },
    };

    // TODO: Clean this up
    let res = if http_method == reqwest::Method::GET {
        client
            .get(api_endpoint.value())
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
            .delete(api_endpoint.value())
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
