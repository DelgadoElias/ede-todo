use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "ede-todo")]
#[command(about = "A simple CLI to learn about Rust", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Add {
        #[arg(short, long)]
        task: String,
    },
    List,
    Complete {
        #[arg(short, long)]
        id: i32,
    },
    Remove {
        #[arg(short, long)]
        id: i32,
    },
    Export {
        #[arg(short, long, default_value = "tasks.json")]
        file: String,
    },
    Import {
        #[arg(short, long)]
        file: String,
        #[clap(long)]
        dry_run: bool,
    },
}
