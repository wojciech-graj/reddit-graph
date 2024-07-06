use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(long)]
    pub db: String,
    #[arg(short, long)]
    pub input: String,
}
