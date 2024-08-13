use clap::{Parser, Subcommand};
use reqwest as request;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

pub mod search;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// LogLevel for tracing
    #[arg(short, long, action = clap::ArgAction::Set)]
    debug_level: Option<String>,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// does testing things
    Test {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },
    Search {
        uri: String,
    },
    Follow,
    Unfollow,
    Publish,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let debug_level = match cli.debug_level.unwrap_or_default().as_str() {
        "error" => Some(Level::ERROR),
        "warn" => Some(Level::WARN),
        "info" => Some(Level::INFO),
        "debug" => Some(Level::DEBUG),
        "trace" => Some(Level::TRACE),
        _ => Some(Level::ERROR),
    };

    let subscriber = FmtSubscriber::builder()
        .with_line_number(true)
        .with_level(true)
        .pretty()
        .with_max_level(debug_level.unwrap())
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Command::Test { list } => {
            if *list {
                println!("Printing testing lists...");
            } else {
                println!("Not printing testing lists...");
            }
        }
        Command::Search { uri } => {
            tracing::error!("I am erroring");
            tracing::warn!("I am waring");
            tracing::info!("I am informing");
            tracing::debug!("I am debugging");
            tracing::trace!("I am tracing");

            tracing::debug!("{:?}", uri);
        }
        Command::Follow => {}
        Command::Unfollow => {}
        Command::Publish => {}
    }
}
/*

kite search @foo@bar.com
  1. WebFinger query
  2. Get an Actor
  3. Get all follows, # of follows
  4. Get all following, # of following
  5. Get all statuses

Kite follow @foo@bar.com
kite unfollow @foo@bar.com

kite publish "Hello World"


*/
