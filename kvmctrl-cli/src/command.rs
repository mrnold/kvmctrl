use clap::ArgAction;
use clap::Args;
use clap::Parser;
use clap::Subcommand;

#[derive(Parser)]
#[command(arg_required_else_help = true)]
pub struct Command {
    #[command(subcommand)]
    pub command: Option<Subcommands>,

    #[arg(short, long, global=true, action=ArgAction::Count)]
    pub debug: u8,
}

#[derive(Subcommand)]
pub enum Subcommands {
    /// Switch all inputs to specified console
    All(PortArgs),
}

#[derive(Args)]
pub struct PortArgs {
    #[arg(short, long, value_parser=clap::value_parser!(u8).range(1..=4))]
    pub port: u8,
}