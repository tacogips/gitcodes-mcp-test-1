use clap::{Parser, Subcommand};
use rust_project_example::{self, create_config};
use std::error::Error;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// API URL to use
    #[arg(short, long)]
    api_url: Option<String>,

    /// API key for authentication
    #[arg(short, long)]
    api_key: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Fetch data from the API
    Fetch {
        /// Resource ID to fetch
        #[arg(short, long)]
        id: String,
    },
    /// List available resources
    List {
        /// Maximum number of items to show
        #[arg(short, long, default_value_t = 10)]
        limit: usize,
    },
    /// Create a new resource
    Create {
        /// Resource name
        #[arg(short, long)]
        name: String,
        
        /// Resource type
        #[arg(short, long)]
        resource_type: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize library
    rust_project_example::initialize();

    // Parse command line arguments
    let cli = Cli::parse();
    
    // Create configuration
    let config = create_config(cli.api_url, cli.api_key);
    
    // Process commands
    match &cli.command {
        Commands::Fetch { id } => {
            println!("Fetching resource with ID: {}", id);
            // Implementation would use the client to fetch data
        }
        Commands::List { limit } => {
            println!("Listing up to {} resources:", limit);
            // Implementation would use the client to list resources
        }
        Commands::Create { name, resource_type } => {
            println!("Creating a new {} resource named: {}", resource_type, name);
            // Implementation would use the client to create a resource
        }
    }

    Ok(())
}