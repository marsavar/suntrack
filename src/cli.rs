use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    pub city: String,
    #[clap(short, long)]
    pub local: bool,
}
