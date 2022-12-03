use chrono::{self, Local};
use clap::{self, Arg, Command};
use env_logger::Builder;
use log::LevelFilter;
use std::fmt;
use std::io::Write;
use std::path::Path;

pub enum InitError {
    ConfigError(String),
}
impl fmt::Display for InitError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ConfigError(val) => write!(fmt, "{}", val),
            _ => write!(fmt, "Error could not be printed, you are on your own"),
        }
    }
}

pub struct AppState {
    port: u16,
}

impl AppState {
    pub fn new(port: u16) -> Self {
        Self { port }
    }
}

fn init() -> Result<AppState, InitError> {
    // Accept command line arguments
    let args = Command::new("freemind server")
        .version(clap::crate_version!())
        .author(clap::crate_authors!("\n"))
        .about(clap::crate_description!())
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .help("Set the port the server listens on for requests")
                .value_name("PORT")
                .default_value("2121"),
        )
        .arg(
            Arg::new("disable-logger")
                .long("disable-logger")
                .help("Disables the default logger")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    if !args.get_flag("disable-logger") {
        // Build the logger
        Builder::new()
            .format(|buf, record| {
                writeln!(
                    buf,
                    "[{}] {} - {}: {}",
                    record.level(),
                    Local::now().format("%d/%m/%y %H:%M:%S"),
                    record.target(),
                    record.args(),
                )
            })
            .filter(None, LevelFilter::Info)
            .init();
    }
    // Return the App state
    let state = AppState::new(*args.get_one("PORT").expect("Invalid Port provided"));
    Ok(state)
}

fn main() {
    // Initialize
    let state = init().unwrap_or_else(|err| {
        log::error!("{}", err);
        panic!("Failed to initialize");
    });

    handler::run(state).unwrap_or_else(|err| {
        log::error!("Failed to start ELAYNE server");
        panic!("{}", err);
    });
    log::info!("Stopped freemind server");
}
