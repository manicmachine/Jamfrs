use crate::args::{ComputerSubcommand, EntityType, MobileSubcommand, UserSubcommand};
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
                (Method::POST, format!("{host}/api/auth/tokens"))
            }
            ApiEndpoints::ComputerDelete { host, id } => (
                Method::DELETE,
                format!("{host}/JSSResource/computers/id/{id}"),
            ),
            ApiEndpoints::ComputerShow { host, id } => (
                Method::GET,
                format!("{host}/JSSResource/computers/id/{id}"),
            ),
            ApiEndpoints::ComputerSearch { host, query_string } => (
                Method::GET,
                format!("{host}/JSSResource/computers/match/{query_string}"),
            ),
            ApiEndpoints::ComputerList(host) => {
                (Method::GET, format!("{host}/JSSResource/computers"))
            }
            ApiEndpoints::MobileDelete { host, id } => (
                Method::DELETE,
                format!("{host}/JSSResource/mobiledevices/id/{id}"),
            ),
            ApiEndpoints::MobileShow { host, id } => (
                Method::GET,
                format!("{host}/JSSResource/mobiledevices/id/{id}"),
            ),
            ApiEndpoints::MobileSearch { host, query_string } => (
                Method::GET,
                format!("{host}/JSSResource/mobiledevices/match/{query_string}"),
            ),
            ApiEndpoints::MobileList(host) => (
                Method::GET,
                format!("{host}/JSSResource/mobiledevices"),
            ),
            ApiEndpoints::UserDelete { host, id } => (
                Method::DELETE,
                format!("{host}/JSSResource/users/id/{id}"),
            ),
            ApiEndpoints::UserShow { host, id } => (
                Method::GET,
                format!("{host}/JSSResource/users/id/{id}"),
            ),
            ApiEndpoints::UserList(host) => {
                (Method::GET, format!("{host}/JSSResource/users"))
            }
        }
    }

    pub fn get_endpoint(entity_type: &EntityType, server_address: &'a String) -> Self {
        match &entity_type {
            EntityType::Computer(command) => match &command.subcommand {
                ComputerSubcommand::Delete { id } => ApiEndpoints::ComputerDelete {
                    host: server_address,
                    id: *id,
                },
                ComputerSubcommand::Show { id } => ApiEndpoints::ComputerShow {
                    host: server_address,
                    id: *id,
                },
                ComputerSubcommand::Search { search_query } => ApiEndpoints::ComputerSearch {
                    host: server_address,
                    query_string: search_query.clone(),
                },
                ComputerSubcommand::List => ApiEndpoints::ComputerList(server_address),
            },
            EntityType::Mobile(command) => match &command.subcommand {
                MobileSubcommand::Delete { id } => ApiEndpoints::MobileDelete {
                    host: server_address,
                    id: *id,
                },
                MobileSubcommand::Show { id } => ApiEndpoints::MobileShow {
                    host: server_address,
                    id: *id,
                },
                MobileSubcommand::Search { search_query } => ApiEndpoints::MobileSearch {
                    host: server_address,
                    query_string: search_query.clone(),
                },
                MobileSubcommand::List => ApiEndpoints::MobileList(server_address),
            },
            EntityType::User(command) => match &command.subcommand {
                UserSubcommand::Delete { id } => ApiEndpoints::UserDelete {
                    host: server_address,
                    id: *id,
                },
                UserSubcommand::Show { id } => ApiEndpoints::UserShow {
                    host: server_address,
                    id: *id,
                },
                UserSubcommand::List => ApiEndpoints::UserList(server_address),
            },
        }
    }
}
