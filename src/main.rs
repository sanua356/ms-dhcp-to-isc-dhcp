use clap::Parser;

mod cli;

mod configs;
mod constants;
mod helpers;
mod transformers;
mod validators;

fn main() {
    let args = cli::CLIArgs::parse();
    cli::run_cli(args);
}
