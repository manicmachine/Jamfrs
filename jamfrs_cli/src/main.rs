mod arg_mappings;
mod args;

use args::JamfrsArgs;
use clap::Parser;
use jamfrs_lib::api_service::JamfApiService;
use reqwest::Method;
use serde_json::Value;
use std::io;
use std::io::stdout;
use std::process::exit;
use xmltree::{Element, EmitterConfig};

#[tokio::main]
async fn main() {
    let args = JamfrsArgs::parse();
    let mut jamf_api_service = match JamfApiService::new(
        args.server_address,
        args.port,
        args.username,
        args.password,
        args.insecure,
        args.json,
    ) {
        Ok(service) => service,
        Err(err) => {
            eprintln!("Failed to create network service: {err}");
            exit(1);
        }
    };

    match jamf_api_service.set_commands(arg_mappings::get_api_details(&args.entity_type).unwrap()) {
        Ok(api_details) => {
            if !args.confirm && api_details.endpoint.method == Method::DELETE {
                let mut input = String::new();
                println!(
                    "Confirm you wish to DELETE {} record(s): (Y/N): ",
                    &jamf_api_service.number_of_commands()
                );

                io::stdin().read_line(&mut input).unwrap();

                if input.to_lowercase().trim().ne("y") {
                    exit(0);
                }
            }
        }
        Err(err) => {
            eprintln!("{err}");
            exit(1);
        }
    }

    let mut rx = jamf_api_service.process_commands().await;
    let mut errors: Vec<String> = Vec::new();

    loop {
        let res = &rx.recv().await;

        match res {
            Some(res) => match res {
                Ok(res) => {
                    if !args.json && args.pretty {
                        let parsed_xml = Element::parse(res.as_bytes()).unwrap();
                        let mut emitter_config = EmitterConfig::new();
                        emitter_config.perform_indent = true;

                        parsed_xml
                            .write_with_config(stdout(), emitter_config)
                            .unwrap();
                    } else if args.json && args.pretty {
                        let json_obj: Result<Value, _> = serde_json::from_str(res);

                        match json_obj {
                            Ok(json) => {
                                println!("{}", serde_json::to_string_pretty(&json).unwrap())
                            }
                            Err(_) => println!("{res}"),
                        }
                    } else {
                        print!("{res},");
                    }
                }
                Err(err) => errors.push(err.to_string()),
            },
            None => {
                // Channel has been closed and we're done
                for err in &errors {
                    println!("\nError: {err}");
                }

                break;
            }
        }
    }
}
