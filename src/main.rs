mod args;
mod db;

use args::{Cli, Commands};
use clap::Parser;
use db::Database;

fn main() {
    let cli = Cli::parse();
    let db = match Database::new() {
        Ok(database) => database,
        Err(e) => {
            eprintln!("❌ Error reaching the database: {}", e);
            return;
        }
    };

    match cli.command {
        Commands::Add { task } => {
            db.add_task(&task).expect("Error adding the task");
            println!("✅ Task added: {}", task);
        }
        Commands::List => {
            let tasks = db.list_tasks().expect("Error listing tasks");
            if tasks.is_empty() {
                println!("📭 No pending tasks.");
            } else {
                println!("📋 Tasks list:");
                for (id, description, completed) in tasks {
                    let status = if completed { "✔️" } else { "❌" };
                    println!("[{}] {} - {}", id, description, status);
                }
            }
        }
        Commands::Complete { id } => {
            db.complete_task(id).expect("Error completing the task");
            println!("✅ Task {} marked as completed", id);
        }
        Commands::Remove { id } => {
            db.remove_task(id).expect("Error deleting the task");
            println!("🗑️ Task {} deleted", id);
        }
        Commands::Export { file } => {
            db.export_to_json(&file).expect("Error exporting JSON");
            println!("📁 Task exported to {}", file);
        }
        Commands::Import { file, dry_run } => {
            if dry_run {
                println!("🔍 Dry run: Processing JSON import {}", file);
                if let Err(e) = db.import_from_json_dry_run(&file) {
                    eprintln!("❌ Error processing JSON import: {}", e);
                } else {
                    println!("✅ Import process succeed.");
                }
            } else {
                if let Err(e) = db.import_from_json(&file) {
                    eprintln!("❌ Error importing tasks from JSON: {}", e);
                } else {
                    println!("📥 Tasks imported from {}", file);
                }
            }
        }
    }
}
