use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct JamfrsArgs {
    /// Hostname or IP address for Jamf Pro server
    #[arg(short, long = "server")]
    pub server_address: String,

    /// Port that the Jamf Pro server is listening to; Defaults to 443 for Jamf cloud instances, 8443 otherwise
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

    /// Request JSON data instead of the default XML
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
    /// Work with policy records
    Policy(PolicyCommand),
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
    #[clap(required = true, value_delimiter = ',')]
    pub id: Vec<u32>,
}
