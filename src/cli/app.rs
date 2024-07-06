use clap::{Command, command, Parser, Subcommand};
use clap::builder::Str;
use crate::cli;


#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
}





pub struct App {
    command: Command
}

pub trait SubCommand {
    fn commands() -> Command;
    fn run_matches(matches: &clap::ArgMatches);
}

impl App {
    pub fn new() -> Self {

        let command = command!()
            .subcommand(
                cli::provider::ProviderCommandManager{}.commands()
            );
        Self {
            command
        }
    }
    pub async fn run(&self) -> std::io::Result<()>{
        let matches = self.command.clone().get_matches();
        match matches.subcommand() {
            Some(("provider", sub_matches)) => {
                cli::provider::run_matches(&sub_matches);
            },
            None => { todo!(start tui) },
            _ => unreachable!("should be unreachable")
        }
        // if let Some(_matches) = matches.subcommand_matches("provider") {
        //     // println!("provider command");
        // }

        Ok(())
    }
}
