use formatx::formatx;
use clap::{Parser, Subcommand};
use mwbot::parsoid::WikiMultinode;

const BOT_CONFIG_PATH: &'static str = "~/.config/mwbot.toml";

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// What action to perform
    #[command(subcommand)]
    action: Action,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Subcommand)]
enum Action {
    /// Move config to ~/.config
    Setup {
        /// Specify the username of the bot
        #[arg(long, env = "MW_USERNAME")]
        username: String,
        /// Specify the botpassword of the bot
        #[arg(long, env = "MW_BOTPASSWORD")]
        botpassword: String,
        /// Specify the OAuth2 token of the bot
        #[arg(long, env = "MW_OAUTH2")]
        oauth2_token: String,
        /// Specify the API URL of the bot
        #[arg(long, env = "MW_API_URL")]
        api_url: String,
        /// Specify the REST URL of the bot
        #[arg(long, env = "MW_REST_URL")]
        rest_url: String,
    },
    /// Run the bot
    Run,
}

fn setup(action: Action) -> Result<(), std::io::Error> {
    if let Ok(config_template) = std::fs::read_to_string("mwbot.toml") {
        match action {
            Action::Run => {panic!();}
            Action::Setup { username, botpassword, oauth2_token, api_url, rest_url } => {
                let filled_in_config = formatx!(config_template, api_url, rest_url, username, botpassword, oauth2_token).unwrap();
                std::fs::write(shellexpand::tilde(BOT_CONFIG_PATH).into_owned(), filled_in_config).unwrap();
            }
        }
        Ok(())
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Unable to find mwbot.toml; did you execute the script from the same directory?"))
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenvy::dotenv().expect("Couldn't load .env file; please make sure to create one in the same directory as executing from!");

    let cli = Cli::parse();

    match cli.action {
        action @ Action::Setup {..} => {
            setup(action)?;
            eprintln!("Successfully set up {BOT_CONFIG_PATH}.")
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
