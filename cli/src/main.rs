use clap::{Parser, Subcommand};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

use crate::search::search;

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
    Search { uri: String },
    Follow { uri: String },
    Unfollow { uri: String },
    Publish { status: String },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let debug_level = match cli.debug_level.unwrap_or_default().as_str() {
        "off" => Some(Level::ERROR),
        "error" => Some(Level::ERROR),
        "warn" => Some(Level::WARN),
        "info" => Some(Level::INFO),
        "debug" => Some(Level::DEBUG),
        "trace" => Some(Level::TRACE),
        "all" => Some(Level::TRACE),
        _ => Some(Level::ERROR),
    };

    let subscriber = FmtSubscriber::builder()
        .with_line_number(true)
        .with_level(true)
        .pretty()
        .with_max_level(debug_level.unwrap())
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Command::Search { uri } => {
            search(uri).await;
        }
        Command::Follow { uri } => {}
        Command::Unfollow { uri } => {}
        Command::Publish { status } => {}
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
