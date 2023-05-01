use reqwest::Method;
use std::collections::HashMap;

#[allow(dead_code)]
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
                url:
                    "/JSSResource/patchreports/patchsoftwaretitleid/{id}/version/{software_version}",
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
}

pub enum Args {
    None,
    Strings(HashMap<&'static str, String>),
    Ids(Vec<String>),
}

pub struct CommandDetails {
    pub args: Args,
    pub endpoint: ApiEndpointDetails,
}

pub struct ApiEndpointDetails {
    pub method: Method,
    pub url: &'static str,
}
