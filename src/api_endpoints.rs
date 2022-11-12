use crate::args::{ComputerSubcommand, EntityType, MobileSubcommand, UserSubcommand};
use crate::JamfrsArgs;
use reqwest::Method;

pub enum ApiEndpoints<'a> {
    // Util
    TokenAuth(&'a String),
    // Computer
    ComputerDelete {
        host: &'a String,
        id: u32,
    },
    ComputerShow {
        host: &'a String,
        id: u32,
    },
    ComputerSearch {
        host: &'a String,
        query_string: String,
    },
    ComputerList(&'a String),
    // Mobile Device
    MobileDelete {
        host: &'a String,
        id: u32,
    },
    MobileShow {
        host: &'a String,
        id: u32,
    },
    MobileSearch {
        host: &'a String,
        query_string: String,
    },
    MobileList(&'a String),
    // User
    UserDelete {
        host: &'a String,
        id: u32,
    },
    UserShow {
        host: &'a String,
        id: u32,
    },
    UserList(&'a String),
}

impl<'a> ApiEndpoints<'a> {
    pub fn usage(&self) -> (reqwest::Method, String) {
        match self {
            ApiEndpoints::TokenAuth(host) => {
                (Method::POST, format!("https://{host}/api/auth/tokens"))
            }
            ApiEndpoints::ComputerDelete { host, id } => (
                Method::DELETE,
                format!("https://{host}/JSSResource/computers/id/{id}"),
            ),
            ApiEndpoints::ComputerShow { host, id } => (
                Method::GET,
                format!("https://{host}/JSSResource/computers/id/{id}"),
            ),
            ApiEndpoints::ComputerSearch { host, query_string } => (
                Method::GET,
                format!("https://{host}/JSSResource/computers/match/{query_string}"),
            ),
            ApiEndpoints::ComputerList(host) => {
                (Method::GET, format!("https://{host}/JSSResource/computers"))
            }
            ApiEndpoints::MobileDelete { host, id } => (
                Method::DELETE,
                format!("https://{host}/JSSResource/mobiledevices/id/{id}"),
            ),
            ApiEndpoints::MobileShow { host, id } => (
                Method::GET,
                format!("https://{host}/JSSResource/mobiledevices/id/{id}"),
            ),
            ApiEndpoints::MobileSearch { host, query_string } => (
                Method::GET,
                format!("https://{host}/JSSResource/mobiledevices/match/{query_string}"),
            ),
            ApiEndpoints::MobileList(host) => (
                Method::GET,
                format!("https://{host}/JSSResource/mobiledevices"),
            ),
            ApiEndpoints::UserDelete { host, id } => (
                Method::DELETE,
                format!("https://{host}/JSSResource/users/id/{id}"),
            ),
            ApiEndpoints::UserShow { host, id } => (
                Method::GET,
                format!("https://{host}/JSSResource/users/id/{id}"),
            ),
            ApiEndpoints::UserList(host) => {
                (Method::GET, format!("https://{host}/JSSResource/users"))
            }
        }
    }

    pub fn get_endpoint_for_args(args: &'a JamfrsArgs) -> Self {
        match &args.entity_type {
            EntityType::Computer(command) => match &command.subcommand {
                ComputerSubcommand::Delete { id } => ApiEndpoints::ComputerDelete {
                    host: &args.server_address,
                    id: *id,
                },
                ComputerSubcommand::Show { id } => ApiEndpoints::ComputerShow {
                    host: &args.server_address,
                    id: *id,
                },
                ComputerSubcommand::Search { search_query } => ApiEndpoints::ComputerSearch {
                    host: &args.server_address,
                    query_string: search_query.clone(),
                },
                ComputerSubcommand::List => ApiEndpoints::ComputerList(&args.server_address),
            },
            EntityType::Mobile(command) => match &command.subcommand {
                MobileSubcommand::Delete { id } => ApiEndpoints::MobileDelete {
                    host: &args.server_address,
                    id: *id,
                },
                MobileSubcommand::Show { id } => ApiEndpoints::MobileShow {
                    host: &args.server_address,
                    id: *id,
                },
                MobileSubcommand::Search { search_query } => ApiEndpoints::MobileSearch {
                    host: &args.server_address,
                    query_string: search_query.clone(),
                },
                MobileSubcommand::List => ApiEndpoints::MobileList(&args.server_address),
            },
            EntityType::User(command) => match &command.subcommand {
                UserSubcommand::Delete { id } => ApiEndpoints::UserDelete {
                    host: &args.server_address,
                    id: *id,
                },
                UserSubcommand::Show { id } => ApiEndpoints::UserShow {
                    host: &args.server_address,
                    id: *id,
                },
                UserSubcommand::List => ApiEndpoints::UserList(&args.server_address),
            },
        }
    }
}
