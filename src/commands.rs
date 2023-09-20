use super::*;

#[derive(Debug, Parser)]
#[clap(
name = "Redash Utils",
about = "Redash Utils is a CLI Application to vacuum large Redash tables and more.",
version = "0.0.1"
)]

pub struct Utils {
    #[clap(subcommand)]
    pub commands: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Vacuum a specific table
    #[clap(arg_required_else_help = true)]
    Vacuum {
        #[clap(short = 't', long)]
        table: String,
    },
}
