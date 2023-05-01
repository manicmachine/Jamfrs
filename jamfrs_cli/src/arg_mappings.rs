use crate::args::*;
use jamfrs_lib::api_service::api_endpoints::{ApiEndpoints, Args, CommandDetails};
use std::collections::HashMap;

pub fn get_command_details(entity_type: &EntityType) -> Result<CommandDetails, String> {
    let mut args_map: HashMap<&str, String> = HashMap::new();

    let (args, endpoint) = match &entity_type {
        EntityType::Computer(command) => match &command.subcommand {
            ComputerSubcommand::Delete(id) => {
                (Args::Ids(id.get_ids()?), ApiEndpoints::ComputerDelete)
            }
            ComputerSubcommand::Show(id) => (Args::Ids(id.get_ids()?), ApiEndpoints::ComputerShow),
            ComputerSubcommand::Search { search_query } => {
                args_map.insert("{search_query}", search_query.clone());
                (Args::Strings(args_map), ApiEndpoints::ComputerSearch)
            }
            ComputerSubcommand::List => (Args::None, ApiEndpoints::ComputerList),
        },
        EntityType::Mobile(command) => match &command.subcommand {
            MobileSubcommand::Delete(id) => (Args::Ids(id.get_ids()?), ApiEndpoints::MobileDelete),
            MobileSubcommand::Show(id) => (Args::Ids(id.get_ids()?), ApiEndpoints::MobileShow),
            MobileSubcommand::Search { search_query } => {
                args_map.insert("{search_query}", search_query.clone());
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
            PolicySubcommand::Delete(id) => (Args::Ids(id.get_ids()?), ApiEndpoints::PolicyDelete),
            PolicySubcommand::Show(id) => (Args::Ids(id.get_ids()?), ApiEndpoints::PolicyShow),
            PolicySubcommand::List => (Args::None, ApiEndpoints::PolicyList),
        },
        EntityType::Package(command) => match &command.subcommand {
            PackageSubcommand::Delete(id) => {
                (Args::Ids(id.get_ids()?), ApiEndpoints::PackageDelete)
            }
            PackageSubcommand::Show(id) => (Args::Ids(id.get_ids()?), ApiEndpoints::PackageShow),
            PackageSubcommand::List => (Args::None, ApiEndpoints::PackageList),
        },
        EntityType::Category(command) => match &command.subcommand {
            CategorySubcommand::Delete(id) => {
                (Args::Ids(id.get_ids()?), ApiEndpoints::CategoryDelete)
            }
            CategorySubcommand::Show(id) => (Args::Ids(id.get_ids()?), ApiEndpoints::CategoryShow),
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
            EbookSubcommand::Delete(id) => (Args::Ids(id.get_ids()?), ApiEndpoints::EbookDelete),
            EbookSubcommand::Show(id) => (Args::Ids(id.get_ids()?), ApiEndpoints::EbookShow),
            EbookSubcommand::List => (Args::None, ApiEndpoints::EbookList),
        },
        EntityType::Building(command) => match &command.subcommand {
            BuildingSubcommand::Delete(id) => {
                (Args::Ids(id.get_ids()?), ApiEndpoints::BuildingDelete)
            }
            BuildingSubcommand::Show(id) => (Args::Ids(id.get_ids()?), ApiEndpoints::BuildingShow),
            BuildingSubcommand::List => (Args::None, ApiEndpoints::BuildingList),
        },
        EntityType::MacApp(command) => match &command.subcommand {
            MacAppSubcommand::Delete(id) => (Args::Ids(id.get_ids()?), ApiEndpoints::MacAppDelete),
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
            ScriptSubcommand::Delete(id) => (Args::Ids(id.get_ids()?), ApiEndpoints::ScriptDelete),
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
            PrinterSubcommand::Show(id) => (Args::Ids(id.get_ids()?), ApiEndpoints::PrinterShow),
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
                    args_map.insert("{id}", id.clone());
                    args_map.insert("{software_version}", software_version.clone());
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
            AdvSearchSubcommand::Computer(adv_search_subcommand) => match &adv_search_subcommand {
                ComputerAdvSearchCommand::Delete(id) => (
                    Args::Ids(id.get_ids()?),
                    ApiEndpoints::AdvSearchComputerDelete,
                ),
                ComputerAdvSearchCommand::Show(id) => (
                    Args::Ids(id.get_ids()?),
                    ApiEndpoints::AdvSearchComputerShow,
                ),
                ComputerAdvSearchCommand::List => (Args::None, ApiEndpoints::AdvSearchComputerList),
            },
            AdvSearchSubcommand::Mobile(adv_search_subcommand) => match &adv_search_subcommand {
                MobileAdvSearchCommand::Delete(id) => (
                    Args::Ids(id.get_ids()?),
                    ApiEndpoints::AdvSearchMobileDelete,
                ),
                MobileAdvSearchCommand::Show(id) => {
                    (Args::Ids(id.get_ids()?), ApiEndpoints::AdvSearchMobileShow)
                }
                MobileAdvSearchCommand::List => (Args::None, ApiEndpoints::AdvSearchMobileList),
            },
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

    Ok(CommandDetails {
        args,
        endpoint: endpoint.usage(),
    })
}
