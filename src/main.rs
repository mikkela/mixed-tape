use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// List all tapes
    List,
    /// Create a new tape
    Create {
        /// The tape length
        length: u8,
    },
    /// Read a tape
    Read {
        /// The tape ID
        id: String,
    },

    /// Update a tape
    Update {
        /// The tape ID
        id: String,
        /// The tape length
        length: u8,
    },
    /// Delete a tape
    Delete {
        /// The tape ID
        id: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let cli = Cli::parse();

    Ok(())
}
