use clap::Parser;

#[derive(Parser)]
#[command(version, about="Advent of Code 2023", long_about = None)]
pub struct Cli {
    #[arg(
        short,
        long,
        help = "Specific day to execute (runs all if not specified, default)."
    )]
    pub day: Option<u8>,
}
