use clap::{Parser, Subcommand};
use serde::Serialize;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    List {
        #[arg(long)]
        provider: Option<String>
    },
    Start {
        provider: String,
        id: String
    },
    Stop {
        provider: String,
        id: String
    },
    Delete {
        provider: String,
        id: String
    },
}

#[derive(Serialize)]
struct CliResult<T> {
    ok: bool,
    data: Option<T>,
    error: Option<String>,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let aggregator = vm_core::build_aggregator_from_env();

    let result = match cli.command {
        Commands::List { provider } => {
            aggregator.list_vms(provider.as_deref()).await
                .map(|vms| serde_json::to_value(vms).unwrap())
        }
        Commands::Start { provider, id } => {
            aggregator.start_vm(&provider, &id).await
                .map(|_| serde_json::json!({
                    "status": "started"
                }))
        }
        _ => unimplemented!(),
    };

    match result {
        Ok(data) => println!("{}", serde_json::to_string(&CliResult {
            ok: true,
            data: Some(data),
            error: None
        }).unwrap()),
        Err(e) => {
            println!("{}", serde_json::to_string(&CliResult::<()> {
                ok: false,
                data: None,
                error: Some(e.to_string())
            }).unwrap());
            std::process::exit(1);
        }
    }
}