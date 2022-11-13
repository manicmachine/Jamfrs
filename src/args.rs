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
    ///Work with mobile device records
    Mobile(MobileCommand),
    /// Work with users records
    User(UserCommand),
}

#[derive(Debug, Args)]
pub struct ComputerCommand {
    #[clap(subcommand)]
    pub subcommand: ComputerSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum ComputerSubcommand {
    /// Delete an existing computer record
    Delete { id: u32 },
    /// Show an existing computer record
    Show { id: u32 },
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
    Delete { id: u32 },
    /// Show an existing mobile device record
    Show { id: u32 },
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
    Delete { id: u32 },
    /// Show an existing user record
    Show { id: u32 },
    /// List all user records
    List,
}
