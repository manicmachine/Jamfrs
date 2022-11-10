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
    pub fn value(&self) -> String {
        match self {
            ApiEndpoints::TokenAuth(host) => format!("https://{host}/api/auth/tokens"),
            ApiEndpoints::ComputerDelete { host, id } => {
                format!("https://{host}/JSSResource/computers/id/{id}")
            }
            ApiEndpoints::ComputerShow { host, id } => {
                format!("https://{host}/JSSResource/computers/id/{id}")
            }
            ApiEndpoints::ComputerSearch { host, query_string } => {
                format!("https://{host}/JSSResource/computers/match/{query_string}")
            }
            ApiEndpoints::ComputerList(host) => format!("https://{host}/JSSResource/computers"),
            ApiEndpoints::MobileDelete { host, id } => {
                format!("https://{host}/JSSResource/mobiledevices/id/{id}")
            }
            ApiEndpoints::MobileShow { host, id } => {
                format!("https://{host}/JSSResource/mobiledevices/id/{id}")
            }
            ApiEndpoints::MobileSearch { host, query_string } => {
                format!("https://{host}/JSSResource/mobiledevices/match/{query_string}")
            }
            ApiEndpoints::MobileList(host) => format!("https://{host}/JSSResource/mobiledevices"),
            ApiEndpoints::UserDelete { host, id } => {
                format!("https://{host}/JSSResource/users/id/{id}")
            }
            ApiEndpoints::UserShow { host, id } => {
                format!("https://{host}/JSSResource/users/id/{id}")
            }
            ApiEndpoints::UserList(host) => format!("https://{host}/JSSResource/computers"),
        }
    }
}
