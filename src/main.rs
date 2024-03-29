#[macro_use]
extern crate lazy_static;

pub mod handler;
pub mod extract;
pub mod parser;
pub mod translate;
pub mod integrity;
pub mod toc;
pub mod track;
pub mod consts;
pub mod util;
pub mod error;
pub mod evaluate;
pub mod response;
pub mod server;
pub mod drive;

use figlet_rs::FIGfont;
use clap::Parser;

use handler::parse_file;
use server::CambiaServer;

/// Program to parse log files generated by various CD ripping software
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to the log file, ignores server mode arguments if present
    #[arg(short, long)]
    path: Option<String>,
    /// Run the server and the web interface
    #[arg(short, long)]
    server: bool,
    /// Specify a port to listen on
    #[arg(long, value_parser = crate::util::port_in_range, default_value = crate::consts::DEFAULT_PORT)]
    pub port: String,
    /// Set the log level
    #[arg(long)]
    pub tracing: Option<String>,
    /// Save the uploaded logs to a directory
    #[arg(long)]
    pub save_logs: bool,
}

// Shuttle does not support feature flags yet
#[cfg(feature = "shuttle")]
#[shuttle_runtime::main]
pub async fn shuttle() -> shuttle_axum::ShuttleAxum {
    let font = FIGfont::standard().unwrap();
    println!("{}", font.convert("cambia").unwrap());
    Ok(CambiaServer::start_shuttle().into())
}

#[cfg(not(feature = "shuttle"))]
#[tokio::main]
pub async fn main() {
    let args = Args::parse();
    
    if args.path.is_some() {
        parse_file(&args.path.unwrap()[..]);
    } else {
        let font = FIGfont::standard().unwrap();
        println!("{}", font.convert("cambia").unwrap());
        CambiaServer::start(args).await;
    }
}
