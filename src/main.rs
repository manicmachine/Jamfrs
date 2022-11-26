mod api_endpoints;
mod api_token;
mod args;
mod api_service;
mod session;

use crate::api_endpoints::{ApiDetails, ApiEndpoints, Args};
use crate::api_service::ApiService;
use crate::session::Session;
use args::JamfrsArgs;
use clap::Parser;
use serde_json::Value;
use std::io::stdout;
use std::process::exit;
use xmltree::{Element, EmitterConfig};

fn main() {
    let args = JamfrsArgs::parse();
    let mut api_service = match ApiService::new(
        args.server_address,
        args.port,
        args.username,
        args.password,
        args.insecure,
        args.json,
    ) {
        Ok(service) => service,
        Err(err) => {
            eprintln!("Failed to create network service: {}", err);
            exit(1);
        }
    };

    api_service.set_commands(&args.entity_type);

    // TODO: Convert this to use an iterator
    let mut i = 0;
    let iterations = api_service.number_of_commands();

    loop {
        let res = api_service.process_command();
        match res {
            Ok(text) => {
                i += 1;

                if !args.json && args.pretty {
                    let parsed_xml = Element::parse(text.as_bytes()).unwrap();
                    let mut emitter_config = EmitterConfig::new();
                    emitter_config.perform_indent = true;

                    parsed_xml
                        .write_with_config(stdout(), emitter_config)
                        .unwrap();
                } else if args.json && args.pretty {
                    let json_obj: Value = serde_json::from_str(&text).unwrap();
                    println!("{}", serde_json::to_string_pretty(&json_obj).unwrap());
                } else if i < iterations {
                    print!("{},", text)
                } else {
                    println!("{}", text);
                }
            }
            Err(err) => {
                eprintln!("Encountered an error: {}", err);
                exit(1);
            }
        }

        if i == iterations {
            break;
        }
    }
}
