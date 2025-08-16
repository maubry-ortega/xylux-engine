use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "xylux", about = "CLI for Xylux Engine")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    New { name: String },
    Run,
    Build { #[arg(long)] target: Option<String> },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::New { name } => println!("Creating project: {}", name),
        Commands::Run => println!("Running Xylux project"),
        Commands::Build { target } => println!("Building for target: {:?}", target),
    }
}