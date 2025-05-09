use std::path::PathBuf;
use formatx::formatx;
use clap::{Parser, Subcommand};
use mwbot::parsoid::WikiMultinode;
use dirs::config_dir;

const CONFIG_TEMPLATE_PATH: &'static str = "mwbot_template.toml";
const BOT_CONFIG_FILE_NAME: &'static str = "mwbot.toml";

fn get_bot_config_path() -> PathBuf {
    config_dir().unwrap().join(BOT_CONFIG_FILE_NAME)
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// What action to perform
    #[command(subcommand)]
    action: Action,
}

#[derive(clap::Args, Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
#[group(required = true, multiple = false)]
struct AuthMethodParse {
    password: Option<String>,
    oauth2_token: Option<String>
}

enum AuthMethod {
    Password(String),
    OAuth2Token(String)
}

impl From<AuthMethodParse> for AuthMethod {
    fn from (parsed_args: AuthMethodParse) -> AuthMethod {
        match parsed_args.password {
            None => Self::OAuth2Token(parsed_args.oauth2_token.unwrap()),
            Some(password) => Self::Password(password)
        }
    }
}

#[derive(Parser, Clone, Eq, PartialEq, PartialOrd, Ord)]
struct SetupArgs {
    /// Specify the username of the bot
    #[arg(long, env = "MW_USERNAME")]
    username: String,
    /// Specify the botpassword of the bot
    #[arg(long, env = "MW_BOTPASSWORD")]
    botpassword: Option<String>,
    /// Specify the OAuth2 token of the bot
    #[arg(long, env = "MW_OAUTH2")]
    oauth2_token: String,
    /// Specify the API URL of the bot
    #[arg(long, env = "MW_API_URL")]
    api_url: String,
    /// Specify the REST URL of the bot
    #[arg(long, env = "MW_REST_URL")]
    rest_url: String,

}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Subcommand)]
enum Action {
    /// Output a bot config to system's config directory
    Setup(SetupArgs),
    /// Run the bot
    Run,
}

fn read_config_template() -> Result<String, std::io::Error> {
    std::fs::read_to_string(CONFIG_TEMPLATE_PATH).or(Err(std::io::Error::new(std::io::ErrorKind::NotFound, format!("Unable to find {}; did you execute the script from the same directory?", CONFIG_TEMPLATE_PATH))))
}

fn fill_config_template(config_template: String, args: SetupArgs) -> String {
    formatx!(config_template, args.api_url, args.rest_url, args.username, args.botpassword.unwrap_or("".into()), args.oauth2_token).unwrap()
}

// Fix permissions on UNIX-like systems, since mwbot-rs doesn't like to read configs with loose permissions.
fn constrain_unix_permissions(path: &PathBuf) -> Result<(), std::io::Error> {
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600))
}

fn setup(args: SetupArgs) -> Result<(), std::io::Error> {
    let config_template = read_config_template()?;
    let filled_in_config = fill_config_template(config_template, args);
    let path = get_bot_config_path();
    std::fs::write(&path, filled_in_config)?;
    constrain_unix_permissions(&path)?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenvy::dotenv().inspect_err(|_| eprintln!("Couldn't load .env file; please make sure to create one in the same directory as executing from!")).ok();

    let cli = Cli::parse();

    match cli.action {
        Action::Setup(args) => {
            setup(args)?;
            eprintln!("Successfully set up {}.", get_bot_config_path().to_str().unwrap())
        },
        Action::Run => {
            let bot = mwbot::Bot::from_default_config().await.unwrap();
            let page = bot.page("Bulgaria").unwrap();
            let html = page.html().await.unwrap().into_mutable();
            println!("{:?}", html);
            println!("{:?}", html.as_nodes());
        }
    }
    Ok(())
}
