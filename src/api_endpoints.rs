use crate::args::{
    AdvSearchSubcommand, BuildingSubcommand, CategorySubcommand, ComputerAdvSearchCommand,
    ComputerGroupCommand, ComputerSubcommand, DepartmentSubcommand, EbookSubcommand, EntityType,
    GroupSubcommand, MacAppSubcommand, MobileAdvSearchCommand, MobileAppSubcommand,
    MobileGroupCommand, MobileSubcommand, PackageSubcommand, PatchAvailableTitleCommand,
    PatchExternalSourceCommand, PatchInternalSourceCommand, PatchPolicyCommand, PatchReportCommand,
    PatchSoftwareTitleCommand, PatchSubcommand, PolicySubcommand, PrinterSubcommand,
    RestrictedSoftwareSubcommand, ScriptSubcommand, UserAdvSearchCommand, UserGroupCommand,
    UserSubcommand,
};
use reqwest::Method;
use std::collections::HashMap;

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
    // Package
    PackageDelete,
    PackageShow,
    PackageList,
    // Category
    CategoryDelete,
    CategoryShow,
    CategoryList,
    // Department
    DepartmentDelete,
    DepartmentShow,
    DepartmentList,
    // EBook
    EbookDelete,
    EbookShow,
    EbookList,
    // Building
    BuildingDelete,
    BuildingShow,
    BuildingList,
    // Mac Applications
    MacAppDelete,
    MacAppShow,
    MacAppList,
    // Mobile Device Applications
    MobileAppDelete,
    MobileAppShow,
    MobileAppList,
    // Scripts
    ScriptDelete,
    ScriptShow,
    ScriptList,
    // Restricted Software
    RestrictedSoftwareDelete,
    RestrictedSoftwareShow,
    RestrictedSoftwareList,
    // Printer
    PrinterDelete,
    PrinterShow,
    PrinterList,
    // Patch
    PatchPolicyDelete,
    PatchPolicyShow,
    PatchPolicyList,
    PatchReportListSoftware,
    PatchReportListComputer,
    PatchSoftwareTitleDelete,
    PatchSoftwareTitleShow,
    PatchSoftwareTitleList,
    PatchAvailableTitleList,
    PatchExternalSourceShow,
    PatchExternalSourceList,
    PatchExternalSourceDelete,
    PatchInternalSourceShow,
    PatchInternalSourceList,
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
                url: "/JSSResource/computers/id/{id}",
            },
            ApiEndpoints::ComputerShow => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/computers/id/{id}",
            },
            ApiEndpoints::ComputerSearch => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/computers/match/{search_query}",
            },
            ApiEndpoints::ComputerList => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/computers",
            },
            ApiEndpoints::MobileDelete => ApiEndpointDetails {
                method: Method::DELETE,
                url: "/JSSResource/mobiledevices/id/{id}",
            },
            ApiEndpoints::MobileShow => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/mobiledevices/id/{id}",
            },
            ApiEndpoints::MobileSearch => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/mobiledevices/match/{search_query}",
            },
            ApiEndpoints::MobileList => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/mobiledevices",
            },
            ApiEndpoints::UserDelete => ApiEndpointDetails {
                method: Method::DELETE,
                url: "/JSSResource/users/id/{id}",
            },
            ApiEndpoints::UserShow => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/users/id/{id}",
            },
            ApiEndpoints::UserList => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/users",
            },
            ApiEndpoints::PolicyDelete => ApiEndpointDetails {
                method: Method::DELETE,
                url: "/JSSResource/policies/id/{id}",
            },
            ApiEndpoints::PolicyShow => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/policies/id/{id}",
            },
            ApiEndpoints::PolicyList => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/policies",
            },
            ApiEndpoints::PackageDelete => ApiEndpointDetails {
                method: Method::DELETE,
                url: "/JSSResource/packages/id/{id}",
            },
            ApiEndpoints::PackageShow => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/packages/id/{id}",
            },
            ApiEndpoints::PackageList => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/packages",
            },
            ApiEndpoints::CategoryDelete => ApiEndpointDetails {
                method: Method::DELETE,
                url: "/JSSResource/categories/id/{id}",
            },
            ApiEndpoints::CategoryShow => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/categories/id/{id}",
            },
            ApiEndpoints::CategoryList => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/categories",
            },
            ApiEndpoints::DepartmentDelete => ApiEndpointDetails {
                method: Method::DELETE,
                url: "/JSSResource/departments/id/{id}",
            },
            ApiEndpoints::DepartmentShow => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/departments/id/{id}",
            },
            ApiEndpoints::DepartmentList => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/departments",
            },
            ApiEndpoints::EbookDelete => ApiEndpointDetails {
                method: Method::DELETE,
                url: "/JSSResource/ebooks/id/{id}",
            },
            ApiEndpoints::EbookShow => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/ebooks/id/{id}",
            },
            ApiEndpoints::EbookList => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/ebooks",
            },
            ApiEndpoints::BuildingDelete => ApiEndpointDetails {
                method: Method::DELETE,
                url: "/JSSResource/buildings/id/{id}",
            },
            ApiEndpoints::BuildingShow => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/buildings/id/{id}",
            },
            ApiEndpoints::BuildingList => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/buildings",
            },
            ApiEndpoints::MacAppDelete => ApiEndpointDetails {
                method: Method::DELETE,
                url: "/JSSResource/macapplications/id/{id}",
            },
            ApiEndpoints::MacAppShow => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/macapplications/id/{id}",
            },
            ApiEndpoints::MacAppList => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/macapplications",
            },
            ApiEndpoints::MobileAppDelete => ApiEndpointDetails {
                method: Method::DELETE,
                url: "/JSSResource/mobiledeviceapplications/id/{id}",
            },
            ApiEndpoints::MobileAppShow => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/mobiledeviceapplications/id/{id}",
            },
            ApiEndpoints::MobileAppList => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/mobiledeviceapplications",
            },
            ApiEndpoints::ScriptDelete => ApiEndpointDetails {
                method: Method::DELETE,
                url: "/JSSResource/scripts/id/{id}",
            },
            ApiEndpoints::ScriptShow => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/scripts/id/{id}",
            },
            ApiEndpoints::ScriptList => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/scripts",
            },
            ApiEndpoints::RestrictedSoftwareDelete => ApiEndpointDetails {
                method: Method::DELETE,
                url: "/JSSResource/restrictedsoftware/id/{id}",
            },
            ApiEndpoints::RestrictedSoftwareShow => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/restrictedsoftware/id/{id}",
            },
            ApiEndpoints::RestrictedSoftwareList => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/restrictedsoftware",
            },
            ApiEndpoints::PrinterDelete => ApiEndpointDetails {
                method: Method::DELETE,
                url: "/JSSResource/printers/id/{id}",
            },
            ApiEndpoints::PrinterShow => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/printers/id/{id}",
            },
            ApiEndpoints::PrinterList => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/printers",
            },
            ApiEndpoints::PatchPolicyDelete => ApiEndpointDetails {
                method: Method::DELETE,
                url: "/JSSResource/patchpolicies/id/{id}",
            },
            ApiEndpoints::PatchPolicyShow => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/patchpolicies/id/{id}",
            },
            ApiEndpoints::PatchPolicyList => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/patchpolicies",
            },
            ApiEndpoints::PatchReportListSoftware => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/patchreports/patchsoftwaretitleid/{id}",
            },
            ApiEndpoints::PatchReportListComputer => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/patchreports/patchsoftwaretitleid/{id}/version/{software_version}",
            },
            ApiEndpoints::PatchSoftwareTitleDelete => ApiEndpointDetails {
                method: Method::DELETE,
                url: "/JSSResource/patchsoftwaretitles/id/{id}",
            },
            ApiEndpoints::PatchSoftwareTitleShow => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/patchsoftwaretitles/id/{id}",
            },
            ApiEndpoints::PatchSoftwareTitleList => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/patchsoftwaretitles",
            },
            ApiEndpoints::PatchAvailableTitleList => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/patchavailabletitles/sourceid/{id}",
            },
            ApiEndpoints::PatchExternalSourceShow => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/patchexternalsources/id/{id}",
            },
            ApiEndpoints::PatchExternalSourceList => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/patchexternalsources",
            },
            ApiEndpoints::PatchExternalSourceDelete => ApiEndpointDetails {
                method: Method::DELETE,
                url: "/JSSResource/patchexternalsources/id/{id}",
            },
            ApiEndpoints::PatchInternalSourceShow => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/patchinternalsources/id/{id}",
            },
            ApiEndpoints::PatchInternalSourceList => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/patchinternalsources",
            },
            ApiEndpoints::GroupComputerDelete => ApiEndpointDetails {
                method: Method::DELETE,
                url: "/JSSResource/computergroups/id/{id}",
            },
            ApiEndpoints::GroupComputerShow => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/computergroups/id/{id}",
            },
            ApiEndpoints::GroupComputerList => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/computergroups",
            },
            ApiEndpoints::GroupMobileDelete => ApiEndpointDetails {
                method: Method::DELETE,
                url: "/JSSResource/mobiledevicegroups/id/{id}",
            },
            ApiEndpoints::GroupMobileShow => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/mobiledevicegroups/id/{id}",
            },
            ApiEndpoints::GroupMobileList => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/mobiledevicegroups",
            },
            ApiEndpoints::GroupUserDelete => ApiEndpointDetails {
                method: Method::DELETE,
                url: "/JSSResource/userroups/id/{id}",
            },
            ApiEndpoints::GroupUserShow => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/usergroups/id/{id}",
            },
            ApiEndpoints::GroupUserList => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/usergroups",
            },
            ApiEndpoints::AdvSearchComputerDelete => ApiEndpointDetails {
                method: Method::DELETE,
                url: "/JSSResource/advancedcomputersearches/id/{id}",
            },
            ApiEndpoints::AdvSearchComputerShow => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/advancedcomputersearches/id/{id}",
            },
            ApiEndpoints::AdvSearchComputerList => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/advancedcomputersearches",
            },
            ApiEndpoints::AdvSearchMobileDelete => ApiEndpointDetails {
                method: Method::DELETE,
                url: "/JSSResource/advancedmobiledevicesearches/id/{id}",
            },
            ApiEndpoints::AdvSearchMobileShow => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/advancedmobiledevicesearches/id/{id}",
            },
            ApiEndpoints::AdvSearchMobileList => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/advancedmobiledevicesearches",
            },
            ApiEndpoints::AdvSearchUserDelete => ApiEndpointDetails {
                method: Method::DELETE,
                url: "/JSSResource/advancedusersearches/id/{id}",
            },
            ApiEndpoints::AdvSearchUserShow => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/advancedusersearches/id/{id}",
            },
            ApiEndpoints::AdvSearchUserList => ApiEndpointDetails {
                method: Method::GET,
                url: "/JSSResource/advancedusersearches",
            },
        }
    }

    pub fn get_api_details(entity_type: &EntityType) -> Result<ApiDetails, String> {
        let mut args_map: HashMap<&str, &String> = HashMap::new();

        let (args, endpoint) = match &entity_type {
            EntityType::Computer(command) => match &command.subcommand {
                ComputerSubcommand::Delete(id) => {
                    (Args::Ids(id.get_ids()?), ApiEndpoints::ComputerDelete)
                }
                ComputerSubcommand::Show(id) => {
                    (Args::Ids(id.get_ids()?), ApiEndpoints::ComputerShow)
                }
                ComputerSubcommand::Search { search_query } => {
                    args_map.insert("{search_query}", search_query);
                    (Args::Strings(args_map), ApiEndpoints::ComputerSearch)
                }
                ComputerSubcommand::List => (Args::None, ApiEndpoints::ComputerList),
            },
            EntityType::Mobile(command) => match &command.subcommand {
                MobileSubcommand::Delete(id) => {
                    (Args::Ids(id.get_ids()?), ApiEndpoints::MobileDelete)
                }
                MobileSubcommand::Show(id) => (Args::Ids(id.get_ids()?), ApiEndpoints::MobileShow),
                MobileSubcommand::Search { search_query } => {
                    args_map.insert("{search_query}", search_query);
                    (Args::Strings(args_map), ApiEndpoints::MobileSearch)
                }
                MobileSubcommand::List => (Args::None, ApiEndpoints::MobileList),
            },
            EntityType::User(command) => match &command.subcommand {
                UserSubcommand::Delete(id) => (Args::Ids(id.get_ids()?), ApiEndpoints::UserDelete),
                UserSubcommand::Show(id) => (Args::Ids(id.get_ids()?), ApiEndpoints::UserShow),
                UserSubcommand::List => (Args::None, ApiEndpoints::UserList),
            },
            EntityType::Policy(command) => match &command.subcommand {
                PolicySubcommand::Delete(id) => {
                    (Args::Ids(id.get_ids()?), ApiEndpoints::PolicyDelete)
                }
                PolicySubcommand::Show(id) => (Args::Ids(id.get_ids()?), ApiEndpoints::PolicyShow),
                PolicySubcommand::List => (Args::None, ApiEndpoints::PolicyList),
            },
            EntityType::Package(command) => match &command.subcommand {
                PackageSubcommand::Delete(id) => {
                    (Args::Ids(id.get_ids()?), ApiEndpoints::PackageDelete)
                }
                PackageSubcommand::Show(id) => {
                    (Args::Ids(id.get_ids()?), ApiEndpoints::PackageShow)
                }
                PackageSubcommand::List => (Args::None, ApiEndpoints::PackageList),
            },
            EntityType::Category(command) => match &command.subcommand {
                CategorySubcommand::Delete(id) => {
                    (Args::Ids(id.get_ids()?), ApiEndpoints::CategoryDelete)
                }
                CategorySubcommand::Show(id) => {
                    (Args::Ids(id.get_ids()?), ApiEndpoints::CategoryShow)
                }
                CategorySubcommand::List => (Args::None, ApiEndpoints::CategoryList),
            },
            EntityType::Department(command) => match &command.subcommand {
                DepartmentSubcommand::Delete(id) => {
                    (Args::Ids(id.get_ids()?), ApiEndpoints::DepartmentDelete)
                }
                DepartmentSubcommand::Show(id) => {
                    (Args::Ids(id.get_ids()?), ApiEndpoints::DepartmentShow)
                }
                DepartmentSubcommand::List => (Args::None, ApiEndpoints::DepartmentList),
            },
            EntityType::Ebook(command) => match &command.subcommand {
                EbookSubcommand::Delete(id) => {
                    (Args::Ids(id.get_ids()?), ApiEndpoints::EbookDelete)
                }
                EbookSubcommand::Show(id) => (Args::Ids(id.get_ids()?), ApiEndpoints::EbookShow),
                EbookSubcommand::List => (Args::None, ApiEndpoints::EbookList),
            },
            EntityType::Building(command) => match &command.subcommand {
                BuildingSubcommand::Delete(id) => {
                    (Args::Ids(id.get_ids()?), ApiEndpoints::BuildingDelete)
                }
                BuildingSubcommand::Show(id) => {
                    (Args::Ids(id.get_ids()?), ApiEndpoints::BuildingShow)
                }
                BuildingSubcommand::List => (Args::None, ApiEndpoints::BuildingList),
            },
            EntityType::MacApp(command) => match &command.subcommand {
                MacAppSubcommand::Delete(id) => {
                    (Args::Ids(id.get_ids()?), ApiEndpoints::MacAppDelete)
                }
                MacAppSubcommand::Show(id) => (Args::Ids(id.get_ids()?), ApiEndpoints::MacAppShow),
                MacAppSubcommand::List => (Args::None, ApiEndpoints::MacAppList),
            },
            EntityType::MobileApp(command) => match &command.subcommand {
                MobileAppSubcommand::Delete(id) => {
                    (Args::Ids(id.get_ids()?), ApiEndpoints::MobileAppDelete)
                }
                MobileAppSubcommand::Show(id) => {
                    (Args::Ids(id.get_ids()?), ApiEndpoints::MobileAppShow)
                }
                MobileAppSubcommand::List => (Args::None, ApiEndpoints::MobileAppList),
            },
            EntityType::Script(command) => match &command.subcommand {
                ScriptSubcommand::Delete(id) => {
                    (Args::Ids(id.get_ids()?), ApiEndpoints::ScriptDelete)
                }
                ScriptSubcommand::Show(id) => (Args::Ids(id.get_ids()?), ApiEndpoints::ScriptShow),
                ScriptSubcommand::List => (Args::None, ApiEndpoints::ScriptList),
            },
            EntityType::RestrictedSoftware(command) => match &command.subcommand {
                RestrictedSoftwareSubcommand::Delete(id) => (
                    Args::Ids(id.get_ids()?),
                    ApiEndpoints::RestrictedSoftwareDelete,
                ),
                RestrictedSoftwareSubcommand::Show(id) => (
                    Args::Ids(id.get_ids()?),
                    ApiEndpoints::RestrictedSoftwareShow,
                ),
                RestrictedSoftwareSubcommand::List => {
                    (Args::None, ApiEndpoints::RestrictedSoftwareList)
                }
            },
            EntityType::Printer(command) => match &command.subcommand {
                PrinterSubcommand::Delete(id) => {
                    (Args::Ids(id.get_ids()?), ApiEndpoints::PrinterDelete)
                }
                PrinterSubcommand::Show(id) => {
                    (Args::Ids(id.get_ids()?), ApiEndpoints::PrinterShow)
                }
                PrinterSubcommand::List => (Args::None, ApiEndpoints::PrinterList),
            },
            EntityType::Patch(command) => match &command.patch_command {
                PatchSubcommand::Policy(policy_subcommand) => match &policy_subcommand {
                    PatchPolicyCommand::Delete(id) => {
                        (Args::Ids(id.get_ids()?), ApiEndpoints::PatchPolicyDelete)
                    }
                    PatchPolicyCommand::Show(id) => {
                        (Args::Ids(id.get_ids()?), ApiEndpoints::PatchPolicyShow)
                    }
                    PatchPolicyCommand::List => (Args::None, ApiEndpoints::PatchPolicyList),
                },
                PatchSubcommand::Report(command) => match &command {
                    PatchReportCommand::ListSoftware(id) => (
                        Args::Ids(id.get_ids()?),
                        ApiEndpoints::PatchReportListSoftware,
                    ),
                    PatchReportCommand::ListComputer {
                        id,
                        software_version,
                    } => {
                        args_map.insert("{id}", id);
                        args_map.insert("{software_version}", software_version);
                        (
                            Args::Strings(args_map),
                            ApiEndpoints::PatchReportListComputer,
                        )
                    }
                },
                PatchSubcommand::SoftwareTitles(command) => match &command {
                    PatchSoftwareTitleCommand::Delete(id) => (
                        Args::Ids(id.get_ids()?),
                        ApiEndpoints::PatchSoftwareTitleDelete,
                    ),
                    PatchSoftwareTitleCommand::Show(id) => (
                        Args::Ids(id.get_ids()?),
                        ApiEndpoints::PatchSoftwareTitleShow,
                    ),
                    PatchSoftwareTitleCommand::List => {
                        (Args::None, ApiEndpoints::PatchSoftwareTitleList)
                    }
                },
                PatchSubcommand::AvailableTitles(command) => match &command {
                    PatchAvailableTitleCommand::List(id) => (
                        Args::Ids(id.get_ids()?),
                        ApiEndpoints::PatchAvailableTitleList,
                    ),
                },
                PatchSubcommand::ExternalSources(command) => match &command {
                    PatchExternalSourceCommand::Delete(id) => (
                        Args::Ids(id.get_ids()?),
                        ApiEndpoints::PatchExternalSourceDelete,
                    ),
                    PatchExternalSourceCommand::Show(id) => (
                        Args::Ids(id.get_ids()?),
                        ApiEndpoints::PatchExternalSourceShow,
                    ),
                    PatchExternalSourceCommand::List => {
                        (Args::None, ApiEndpoints::PatchExternalSourceList)
                    }
                },
                PatchSubcommand::InternalSources(command) => match &command {
                    PatchInternalSourceCommand::Show(id) => (
                        Args::Ids(id.get_ids()?),
                        ApiEndpoints::PatchInternalSourceShow,
                    ),
                    PatchInternalSourceCommand::List => {
                        (Args::None, ApiEndpoints::PatchInternalSourceList)
                    }
                },
            },
            EntityType::Group(command) => match &command.group_command {
                GroupSubcommand::Computer(group_subcommand) => match &group_subcommand {
                    ComputerGroupCommand::Delete(id) => {
                        (Args::Ids(id.get_ids()?), ApiEndpoints::GroupComputerDelete)
                    }
                    ComputerGroupCommand::Show(id) => {
                        (Args::Ids(id.get_ids()?), ApiEndpoints::GroupComputerShow)
                    }
                    ComputerGroupCommand::List => (Args::None, ApiEndpoints::GroupComputerList),
                },
                GroupSubcommand::Mobile(group_subcommand) => match &group_subcommand {
                    MobileGroupCommand::Delete(id) => {
                        (Args::Ids(id.get_ids()?), ApiEndpoints::GroupMobileDelete)
                    }
                    MobileGroupCommand::Show(id) => {
                        (Args::Ids(id.get_ids()?), ApiEndpoints::GroupMobileShow)
                    }
                    MobileGroupCommand::List => (Args::None, ApiEndpoints::GroupMobileList),
                },
                GroupSubcommand::User(group_subcommand) => match &group_subcommand {
                    UserGroupCommand::Delete(id) => {
                        (Args::Ids(id.get_ids()?), ApiEndpoints::GroupUserDelete)
                    }
                    UserGroupCommand::Show(id) => {
                        (Args::Ids(id.get_ids()?), ApiEndpoints::GroupUserShow)
                    }
                    UserGroupCommand::List => (Args::None, ApiEndpoints::GroupUserList),
                },
            },
            EntityType::AdvSearch(command) => match &command.adv_search_command {
                AdvSearchSubcommand::Computer(adv_search_subcommand) => {
                    match &adv_search_subcommand {
                        ComputerAdvSearchCommand::Delete(id) => (
                            Args::Ids(id.get_ids()?),
                            ApiEndpoints::AdvSearchComputerDelete,
                        ),
                        ComputerAdvSearchCommand::Show(id) => (
                            Args::Ids(id.get_ids()?),
                            ApiEndpoints::AdvSearchComputerShow,
                        ),
                        ComputerAdvSearchCommand::List => {
                            (Args::None, ApiEndpoints::AdvSearchComputerList)
                        }
                    }
                }
                AdvSearchSubcommand::Mobile(adv_search_subcommand) => {
                    match &adv_search_subcommand {
                        MobileAdvSearchCommand::Delete(id) => (
                            Args::Ids(id.get_ids()?),
                            ApiEndpoints::AdvSearchMobileDelete,
                        ),
                        MobileAdvSearchCommand::Show(id) => {
                            (Args::Ids(id.get_ids()?), ApiEndpoints::AdvSearchMobileShow)
                        }
                        MobileAdvSearchCommand::List => {
                            (Args::None, ApiEndpoints::AdvSearchMobileList)
                        }
                    }
                }
                AdvSearchSubcommand::User(adv_search_subcommand) => match &adv_search_subcommand {
                    UserAdvSearchCommand::Delete(id) => {
                        (Args::Ids(id.get_ids()?), ApiEndpoints::AdvSearchUserDelete)
                    }
                    UserAdvSearchCommand::Show(id) => {
                        (Args::Ids(id.get_ids()?), ApiEndpoints::AdvSearchUserShow)
                    }
                    UserAdvSearchCommand::List => (Args::None, ApiEndpoints::AdvSearchUserList),
                },
            },
        };

        Ok(ApiDetails {
            args,
            endpoint: endpoint.usage(),
        })
    }
}

pub enum Args<'a> {
    None,
    Strings(HashMap<&'static str, &'a String>),
    Ids(Vec<String>),
}

pub struct ApiDetails<'a> {
    pub args: Args<'a>,
    pub endpoint: ApiEndpointDetails,
}

pub struct ApiEndpointDetails {
    pub method: Method,
    pub url: &'static str,
}