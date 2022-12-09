use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct JamfrsArgs {
    /// Hostname or IP address for Jamf Pro server
    #[arg(short, long = "server")]
    pub server_address: String,

    /// Port that the Jamf Pro server is listening to; Defaults to 443 for Jamf cloud instances, 8443 for others. If 'insecure' is passed then the default is 8080.
    #[arg(long)]
    pub port: Option<u16>,

    /// Username used for API calls
    #[arg(short, long = "user")]
    pub username: String,

    /// Password used by API user
    #[arg(short, long)]
    pub password: String,

    /// Pretty print output
    #[arg(long)]
    pub pretty: bool,

    /// Request JSON data instead of the default XML; Note that delete queries always respond with XML
    #[arg(long)]
    pub json: bool,

    /// Allow insecure traffic; Defaults to False. Useful with HTTP or untrusted SSL certificates
    #[arg(long)]
    pub insecure: bool,

    #[command(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// Work with computer records
    Computer(ComputerCommand),
    /// Work with mobile device records
    Mobile(MobileCommand),
    /// Work with users records
    User(UserCommand),
    /// Work with policies
    Policy(PolicyCommand),
    /// Work with packages
    Package(PackageCommand),
    /// Work with categories
    Category(CategoryCommand),
    /// Work with departments
    Department(DepartmentCommand),
    /// Work with ebooks
    Ebook(EbookCommand),
    /// Work with buildings
    Building(BuildingCommand),
    /// Work with mac applications
    MacApp(MacAppCommand),
    /// Work with mobile device applications
    MobileApp(MobileAppCommand),
    /// Work with scripts
    Script(ScriptCommand),
    /// Work with smart & static groups
    Group(GroupCommand),
    /// Work with advanced searches
    AdvSearch(AdvSearchCommand),
}

#[derive(Debug, Args)]
pub struct ComputerCommand {
    #[clap(subcommand)]
    pub subcommand: ComputerSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum ComputerSubcommand {
    /// Delete an existing computer record
    Delete(Id),
    /// Show an existing computer record
    Show(Id),
    /// Search for existing computer records by name, MAC address, username, etc; Accepts * wildcard
    /// Note: Wildcard searches must be wrapped in quotes or escaped to prevent shell expansion
    Search { search_query: String },
    /// List all computers
    List,
}

#[derive(Debug, Args)]
pub struct MobileCommand {
    #[clap(subcommand)]
    pub subcommand: MobileSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum MobileSubcommand {
    /// Delete an existing mobile device record
    Delete(Id),
    /// Show an existing mobile device record
    Show(Id),
    /// Search for existing mobile device records by name, MAC address, username, etc. Accepts * wildcard
    //  Note: Wildcard searches must be wrapped in quotes or escaped to prevent shell expansion
    Search { search_query: String },
    /// List all mobile device records
    List,
}

#[derive(Debug, Args)]
pub struct UserCommand {
    #[clap(subcommand)]
    pub subcommand: UserSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum UserSubcommand {
    /// Delete an existing user record
    Delete(Id),
    /// Show an existing user record
    Show(Id),
    /// List all user records
    List,
}

#[derive(Debug, Args)]
pub struct PolicyCommand {
    #[clap(subcommand)]
    pub subcommand: PolicySubcommand,
}

#[derive(Debug, Subcommand)]
pub enum PolicySubcommand {
    /// Delete an existing policy record
    Delete(Id),
    /// Show an existing policy record
    Show(Id),
    /// List all policy records
    List,
}

#[derive(Debug, Args)]
pub struct PackageCommand {
    #[clap(subcommand)]
    pub subcommand: PackageSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum PackageSubcommand {
    /// Delete an existing package record
    Delete(Id),
    /// Show an existing package record
    Show(Id),
    /// List all package records
    List,
}

#[derive(Debug, Args)]
pub struct CategoryCommand {
    #[clap(subcommand)]
    pub subcommand: CategorySubcommand,
}

#[derive(Debug, Subcommand)]
pub enum CategorySubcommand {
    /// Delete an existing category record
    Delete(Id),
    /// Show an existing category record
    Show(Id),
    /// List all category records
    List,
}

#[derive(Debug, Args)]
pub struct DepartmentCommand {
    #[clap(subcommand)]
    pub subcommand: DepartmentSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum DepartmentSubcommand {
    /// Delete an existing department record
    Delete(Id),
    /// Show an existing department record
    Show(Id),
    /// List all department records
    List,
}

#[derive(Debug, Args)]
pub struct EbookCommand {
    #[clap(subcommand)]
    pub subcommand: EbookSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum EbookSubcommand {
    /// Delete an existing ebook record
    Delete(Id),
    /// Show an existing ebook record
    Show(Id),
    /// List all ebook records
    List,
}

#[derive(Debug, Args)]
pub struct BuildingCommand {
    #[clap(subcommand)]
    pub subcommand: BuildingSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum BuildingSubcommand {
    /// Delete an existing buildings record
    Delete(Id),
    /// Show an existing buildings record
    Show(Id),
    /// List all buildings records
    List,
}

#[derive(Debug, Args)]
pub struct MacAppCommand {
    #[clap(subcommand)]
    pub subcommand: MacAppSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum MacAppSubcommand {
    /// Delete an existing mac application record
    Delete(Id),
    /// Show an existing mac application record
    Show(Id),
    /// List all mac application records
    List,
}

#[derive(Debug, Args)]
pub struct MobileAppCommand {
    #[clap(subcommand)]
    pub subcommand: MobileAppSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum MobileAppSubcommand {
    /// Delete an existing mobile device application record
    Delete(Id),
    /// Show an existing mobile device application record
    Show(Id),
    /// List all mobile device application records
    List,
}

#[derive(Debug, Args)]
pub struct ScriptCommand {
    #[clap(subcommand)]
    pub subcommand: ScriptSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum ScriptSubcommand {
    /// Delete an existing script
    Delete(Id),
    /// Show an existing script
    Show(Id),
    /// List all script
    List,
}

#[derive(Debug, Parser)]
pub struct GroupCommand {
    #[clap(subcommand)]
    pub group_command: GroupSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum GroupSubcommand {
    #[clap(subcommand)]
    /// Work with computer groups
    Computer(ComputerGroupCommand),
    #[clap(subcommand)]
    /// Work with mobile device groups
    Mobile(MobileGroupCommand),
    #[clap(subcommand)]
    /// Work with user groups
    User(UserGroupCommand),
}

#[derive(Debug, Subcommand)]
pub enum ComputerGroupCommand {
    /// Delete an existing computer group
    Delete(Id),
    /// Show an existing computer group
    Show(Id),
    /// List all computer groups
    List,
}

#[derive(Debug, Subcommand)]
pub enum MobileGroupCommand {
    /// Delete an existing mobile device group
    Delete(Id),
    /// Show an existing mobile device group
    Show(Id),
    /// List all mobile device groups
    List,
}

#[derive(Debug, Subcommand)]
pub enum UserGroupCommand {
    /// Delete an existing user group
    Delete(Id),
    /// Show an existing user group
    Show(Id),
    /// List all user groups
    List,
}

#[derive(Debug, Parser)]
pub struct AdvSearchCommand {
    #[clap(subcommand)]
    pub adv_search_command: AdvSearchSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum AdvSearchSubcommand {
    #[clap(subcommand)]
    /// Work with computer advanced searches
    Computer(ComputerAdvSearchCommand),
    #[clap(subcommand)]
    /// Work with mobile device advanced searches
    Mobile(MobileAdvSearchCommand),
    #[clap(subcommand)]
    /// Work with user advanced searches
    User(UserAdvSearchCommand),
}

#[derive(Debug, Subcommand)]
pub enum ComputerAdvSearchCommand {
    /// Delete an existing computer advanced search
    Delete(Id),
    /// Show an existing computer advanced search
    Show(Id),
    /// List all computer advanced searches
    List,
}

#[derive(Debug, Subcommand)]
pub enum MobileAdvSearchCommand {
    /// Delete an existing mobile device advanced search
    Delete(Id),
    /// Show an existing mobile device advanced search
    Show(Id),
    /// List all mobile device advanced searches
    List,
}

#[derive(Debug, Subcommand)]
pub enum UserAdvSearchCommand {
    /// Delete an existing user advanced search
    Delete(Id),
    /// Show an existing user advanced search
    Show(Id),
    /// List all user advanced searches
    List,
}

#[derive(Debug, Args)]
pub struct Id {
    #[arg(value_delimiter = ',', group = "id_range")]
    pub id: Vec<u32>,

    /// Query a range of Ids with the format START,FINISH inclusive
    #[arg(short, long, value_delimiter = ',', group = "id_range", value_parser = range_validator)]
    pub range: Vec<u32>,
}

fn range_validator(s: &str) -> Result<u32, String> {
    static mut RANGE_ARG_COUNT: u32 = 0;
    let arg: Result<u32, _> = s.to_string().parse();

    unsafe {
        if let Ok(arg) = arg {
            if RANGE_ARG_COUNT < 2 {
                RANGE_ARG_COUNT += 1;
                return Ok(arg);
            }
        }
    }
    Err("range only accepts 2 numerical values <START, STOP> inclusive".to_string())
}

impl Id {
    pub fn get_ids(&self) -> Result<Vec<u32>, String> {
        let ids = if !self.id.is_empty() {
            self.id.clone()
        } else {
            self.generate_ids()?
        };

        Ok(ids)
    }

    fn generate_ids(&self) -> Result<Vec<u32>, String> {
        if let (Some(start), Some(end)) = (self.range.first(), self.range.get(1)) {
            Ok((*start..=*end).collect::<Vec<u32>>())
        } else {
            Err("Range requires 2 numerical values <START, STOP> inclusive".to_string())
        }
    }
}
