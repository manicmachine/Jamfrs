use crate::args::{AdvSearchSubcommand, CategorySubcommand, ComputerAdvSearchCommand, ComputerGroupCommand, ComputerSubcommand, EntityType, GroupSubcommand, MobileAdvSearchCommand, MobileGroupCommand, MobileSubcommand, PackageSubcommand, PolicySubcommand, UserAdvSearchCommand, UserGroupCommand, UserSubcommand};
use reqwest::Method;

pub enum ApiEndpoints {
    // Util
    TokenAuth,
    // Computer
    ComputerDelete,
    ComputerShow,
    ComputerSearch,
    ComputerList,
    // Mobile Device
    MobileDelete,
    MobileShow,
    MobileSearch,
    MobileList,
    // User
    UserDelete,
    UserShow,
    UserList,
    // Policy
    PolicyDelete,
    PolicyShow,
    PolicyList,
    /// Package
    PackageDelete,
    PackageShow,
    PackageList,
    /// Category
    CategoryDelete,
    CategoryShow,
    CategoryList,
    // Groups
    GroupComputerDelete,
    GroupComputerShow,
    GroupComputerList,
    GroupMobileDelete,
    GroupMobileShow,
    GroupMobileList,
    GroupUserDelete,
    GroupUserShow,
    GroupUserList,
    // Advanced searches
    AdvSearchComputerDelete,
    AdvSearchComputerShow,
    AdvSearchComputerList,
    AdvSearchMobileDelete,
    AdvSearchMobileShow,
    AdvSearchMobileList,
    AdvSearchUserDelete,
    AdvSearchUserShow,
    AdvSearchUserList,
}

impl ApiEndpoints {
    pub fn usage(&self) -> ApiEndpointDetails {
        match &self {
            ApiEndpoints::TokenAuth => ApiEndpointDetails {
                method: Method::POST,
                url: "/api/auth/tokens",
            },
            ApiEndpoints::ComputerDelete => ApiEndpointDetails {
                method: Method::DELETE,
                url: "/JSSResource/computers/id/{val}",
            },
            ApiEndpoints::ComputerShow => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/computers/id/{val}",
            },
            ApiEndpoints::ComputerSearch => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/computers/match/{val}",
            },
            ApiEndpoints::ComputerList => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/computers",
            },
            ApiEndpoints::MobileDelete => ApiEndpointDetails {
                method: Method::DELETE,
                url: "/JSSResource/mobiledevices/id/{val}",
            },
            ApiEndpoints::MobileShow => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/mobiledevices/id/{val}",
            },
            ApiEndpoints::MobileSearch => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/mobiledevices/match/{val}",
            },
            ApiEndpoints::MobileList => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/mobiledevices",
            },
            ApiEndpoints::UserDelete => ApiEndpointDetails {
                method: Method::DELETE,
                url: "/JSSResource/users/id/{val}",
            },
            ApiEndpoints::UserShow => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/users/id/{val}",
            },
            ApiEndpoints::UserList => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/users",
            },
            ApiEndpoints::PolicyDelete => ApiEndpointDetails {
                method: Method::DELETE,
                url: "/JSSResource/policies/id/{val}",
            },
            ApiEndpoints::PolicyShow => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/policies/id/{val}",
            },
            ApiEndpoints::PolicyList => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/policies",
            },
            ApiEndpoints::PackageDelete => ApiEndpointDetails {
                method: Method::DELETE,
                url: "/JSSResource/packages/id/{val}",
            },
            ApiEndpoints::PackageShow => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/packages/id/{val}",
            },
            ApiEndpoints::PackageList => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/packages",
            },
            ApiEndpoints::CategoryDelete => ApiEndpointDetails {
                method: Method::DELETE,
                url: "/JSSResource/categories/id/{val}",
            },
            ApiEndpoints::CategoryShow => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/categories/id/{val}",
            },
            ApiEndpoints::CategoryList => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/categories",
            },
            ApiEndpoints::GroupComputerDelete => ApiEndpointDetails {
                method: Method::DELETE,
                url: "/JSSResource/computergroups/id/{val}",
            },
            ApiEndpoints::GroupComputerShow => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/computergroups/id/{val}",
            },
            ApiEndpoints::GroupComputerList => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/computergroups",
            },
            ApiEndpoints::GroupMobileDelete => ApiEndpointDetails {
                method: Method::DELETE,
                url: "/JSSResource/mobiledevicegroups/id/{val}",
            },
            ApiEndpoints::GroupMobileShow => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/mobiledevicegroups/id/{val}",
            },
            ApiEndpoints::GroupMobileList => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/mobiledevicegroups",
            },
            ApiEndpoints::GroupUserDelete => ApiEndpointDetails {
                method: Method::DELETE,
                url: "/JSSResource/userroups/id/{val}",
            },
            ApiEndpoints::GroupUserShow => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/usergroups/id/{val}",
            },
            ApiEndpoints::GroupUserList => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/usergroups",
            },
            ApiEndpoints::AdvSearchComputerDelete => ApiEndpointDetails {
                method: Method::DELETE,
                url: "/JSSResource/advancedcomputersearches/id/{val}",
            },
            ApiEndpoints::AdvSearchComputerShow => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/advancedcomputersearches/id/{val}",
            },
            ApiEndpoints::AdvSearchComputerList => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/advancedcomputersearches",
            },
            ApiEndpoints::AdvSearchMobileDelete => ApiEndpointDetails {
                method: Method::DELETE,
                url: "/JSSResource/advancedmobiledevicesearches/id/{val}",
            },
            ApiEndpoints::AdvSearchMobileShow => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/advancedmobiledevicesearches/id/{val}",
            },
            ApiEndpoints::AdvSearchMobileList => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/advancedmobiledevicesearches",
            },
            ApiEndpoints::AdvSearchUserDelete => ApiEndpointDetails {
                method: Method::DELETE,
                url: "/JSSResource/advancedusersearches/id/{val}",
            },
            ApiEndpoints::AdvSearchUserShow => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/advancedusersearches/id/{val}",
            },
            ApiEndpoints::AdvSearchUserList => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/advancedusersearches",
            },
        }
    }

    pub fn get_api_details(entity_type: &EntityType) -> ApiDetails {
        let (args, endpoint) = match &entity_type {
            EntityType::Computer(command) => match &command.subcommand {
                ComputerSubcommand::Delete(id) => (Args::Ids(&id.id), ApiEndpoints::ComputerDelete),
                ComputerSubcommand::Show(id) => (Args::Ids(&id.id), ApiEndpoints::ComputerShow),
                ComputerSubcommand::Search { search_query } => {
                    (Args::String(search_query), ApiEndpoints::ComputerSearch)
                }
                ComputerSubcommand::List => (Args::None, ApiEndpoints::ComputerList),
            },
            EntityType::Mobile(command) => match &command.subcommand {
                MobileSubcommand::Delete(id) => (Args::Ids(&id.id), ApiEndpoints::MobileDelete),
                MobileSubcommand::Show(id) => (Args::Ids(&id.id), ApiEndpoints::MobileShow),
                MobileSubcommand::Search { search_query } => {
                    (Args::String(search_query), ApiEndpoints::MobileSearch)
                }
                MobileSubcommand::List => (Args::None, ApiEndpoints::MobileList),
            },
            EntityType::User(command) => match &command.subcommand {
                UserSubcommand::Delete(id) => (Args::Ids(&id.id), ApiEndpoints::UserDelete),
                UserSubcommand::Show(id) => (Args::Ids(&id.id), ApiEndpoints::UserShow),
                UserSubcommand::List => (Args::None, ApiEndpoints::UserList),
            },
            EntityType::Policy(command) => match &command.subcommand {
                PolicySubcommand::Delete(id) => (Args::Ids(&id.id), ApiEndpoints::PolicyDelete),
                PolicySubcommand::Show(id) => (Args::Ids(&id.id), ApiEndpoints::PolicyShow),
                PolicySubcommand::List => (Args::None, ApiEndpoints::PolicyList),
            },
            EntityType::Package(command) => match &command.subcommand {
                PackageSubcommand::Delete(id) => (Args::Ids(&id.id), ApiEndpoints::PackageDelete),
                PackageSubcommand::Show(id) => (Args::Ids(&id.id), ApiEndpoints::PackageShow),
                PackageSubcommand::List => (Args::None, ApiEndpoints::PackageList),
            },
            EntityType::Category(command) => match &command.subcommand {
                CategorySubcommand::Delete(id) => (Args::Ids(&id.id), ApiEndpoints::CategoryDelete),
                CategorySubcommand::Show(id) => (Args::Ids(&id.id), ApiEndpoints::CategoryShow),
                CategorySubcommand::List => (Args::None, ApiEndpoints::CategoryList),
            },
            EntityType::Group(command) => match &command.group_command {
                GroupSubcommand::Computer(group_subcommand) => match &group_subcommand {
                    ComputerGroupCommand::Delete(id) => {
                        (Args::Ids(&id.id), ApiEndpoints::GroupComputerDelete)
                    }
                    ComputerGroupCommand::Show(id) => {
                        (Args::Ids(&id.id), ApiEndpoints::GroupComputerShow)
                    }
                    ComputerGroupCommand::List => (Args::None, ApiEndpoints::GroupComputerList),
                },
                GroupSubcommand::Mobile(group_subcommand) => match &group_subcommand {
                    MobileGroupCommand::Delete(id) => {
                        (Args::Ids(&id.id), ApiEndpoints::GroupMobileDelete)
                    }
                    MobileGroupCommand::Show(id) => {
                        (Args::Ids(&id.id), ApiEndpoints::GroupMobileShow)
                    }
                    MobileGroupCommand::List => (Args::None, ApiEndpoints::GroupMobileList),
                },
                GroupSubcommand::User(group_subcommand) => match &group_subcommand {
                    UserGroupCommand::Delete(id) => {
                        (Args::Ids(&id.id), ApiEndpoints::GroupUserDelete)
                    }
                    UserGroupCommand::Show(id) => (Args::Ids(&id.id), ApiEndpoints::GroupUserShow),
                    UserGroupCommand::List => (Args::None, ApiEndpoints::GroupUserList),
                },
            },
            EntityType::AdvSearch(command) => match &command.adv_search_command {
                AdvSearchSubcommand::Computer(adv_search_subcommand) => {
                    match &adv_search_subcommand {
                        ComputerAdvSearchCommand::Delete(id) => {
                            (Args::Ids(&id.id), ApiEndpoints::AdvSearchComputerDelete)
                        }
                        ComputerAdvSearchCommand::Show(id) => {
                            (Args::Ids(&id.id), ApiEndpoints::AdvSearchComputerShow)
                        }
                        ComputerAdvSearchCommand::List => {
                            (Args::None, ApiEndpoints::AdvSearchComputerList)
                        }
                    }
                }
                AdvSearchSubcommand::Mobile(adv_search_subcommand) => {
                    match &adv_search_subcommand {
                        MobileAdvSearchCommand::Delete(id) => {
                            (Args::Ids(&id.id), ApiEndpoints::AdvSearchMobileDelete)
                        }
                        MobileAdvSearchCommand::Show(id) => {
                            (Args::Ids(&id.id), ApiEndpoints::AdvSearchMobileShow)
                        }
                        MobileAdvSearchCommand::List => {
                            (Args::None, ApiEndpoints::AdvSearchMobileList)
                        }
                    }
                }
                AdvSearchSubcommand::User(adv_search_subcommand) => match &adv_search_subcommand {
                    UserAdvSearchCommand::Delete(id) => {
                        (Args::Ids(&id.id), ApiEndpoints::AdvSearchUserDelete)
                    }
                    UserAdvSearchCommand::Show(id) => {
                        (Args::Ids(&id.id), ApiEndpoints::AdvSearchUserShow)
                    }
                    UserAdvSearchCommand::List => (Args::None, ApiEndpoints::AdvSearchUserList),
                },
            },
        };

        ApiDetails {
            args,
            endpoint: endpoint.usage(),
        }
    }
}

pub enum Args<'a> {
    None,
    String(&'a String),
    Ids(&'a Vec<u32>),
}

pub struct ApiDetails<'a> {
    pub args: Args<'a>,
    pub endpoint: ApiEndpointDetails,
}

pub struct ApiEndpointDetails {
    pub method: Method,
    pub url: &'static str,
}
