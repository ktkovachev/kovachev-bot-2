use std::process::exit;
use std::env;
use formatx::formatx;

use clap::{Parser, Subcommand};
use mwbot::parsoid::WikiMultinode;

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
    Setup,
    /// Run the bot
    Run {
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
}

fn setup() -> Result<(), &'static str> {
    if let Ok(config_template) = std::fs::read_to_string("mwbot.toml") {

        let username = env::var("MW_USERNAME").expect("The .env file must have a \"MW_USERNAME\" attribute.");
        let oauth2_token = env::var("MW_OAUTH2").expect("The .env file must have a \"MW_OAUTH2\" attribute.");
        let botpassword = env::var("MW_BOTPASSWORD").expect("The .env file must have a \"MW_BOTPASSWORD\" attribute.");

        let filled_in_config = formatx!(config_template, username, botpassword, oauth2_token).unwrap();
        std::fs::write(shellexpand::tilde("~/.config/mwbot.toml").into_owned(), filled_in_config).unwrap();

        Ok(())
    } else {
        Err("Unable to find mwbot.toml; did you execute the script from the same directory?")
    }
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Couldn't load .env file; please make sure to create one in the same directory as executing from!");

    let cli = Cli::parse();

    match cli.action {
        Action::Setup => {
            if let Err(err) = setup() {
                eprintln!("{}", err);
                exit(1);
            } else {
                eprintln!("Successfully set up ~/.config/mwbot.toml.");
            }
        },
        Action::Run {username, botpassword, oauth2_token, api_url, rest_url} => {
            let bot = mwbot::Bot::builder(api_url, rest_url)
                .set_botpassword(username, botpassword)
                .build()
                .await.unwrap();
            let page = bot.page("Bulgaria").unwrap();
            let html = page.html().await.unwrap().into_mutable();
            println!("{:?}", html);
            println!("{:?}", html.as_nodes());
        }
    }
}
