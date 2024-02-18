use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct GraphyArgs {
    /// Path to the database
    #[arg(long)]
    pub db_path: String,
}

