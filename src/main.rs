use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rally-inbox", about = "Rally webhook inbox")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Run the HTTP server on PORT (default 3000).
    Serve,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    match Cli::parse().command {
        Command::Serve => {
            let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
            let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await?;
            axum::serve(listener, rally_inbox::app()).await
        }
    }
}
