use std::process::Command;
use clap::{Parser, Subcommand};
mod commands;
use commands::Utils;
mod vacuum;
use vacuum::Vacuum;

fn main() {
    let utils = Utils::parse();

    match utils.commands {
        commands::Commands::Vacuum { table } => {
            match table.as_str() {
                "users" | "events" | "queries" | "query_results" => {
                    Vacuum::execute(table);
                }
                _ => {
                    eprintln!("Invalid table {}", table);
                }
            }
        }
    }
}

