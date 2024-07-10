use clap::{command, Parser, Subcommand};
use super::provider::ProviderCommands;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {

    /// manage providers
    #[command(subcommand)]
    pub(crate) command: CliCommands
}


#[derive(Subcommand)]
pub enum CliCommands {
    /// manage and view providers
    #[command(subcommand)]
    Provider(ProviderCommands)
}



// pub struct App {
//     command: Command
// }
//
//
//
//
// pub trait SubCommand {
//     fn commands() -> Command;
//     fn run_matches(matches: &clap::ArgMatches);
// }
//
// impl App {
//     pub fn new() -> Self {
//
//         let command = command!()
//             .subcommand(
//                 cli::provider_api::ProviderCommandManager{}.commands()
//             );
//         Self {
//             command
//         }
//     }
//     pub async fn run(&self) -> std::io::Result<()>{
//         let matches = self.command.clone().get_matches();
//         match matches.subcommand() {
//             Some(("provider_api", sub_matches)) => {
//                 cli::provider_api::run_matches(&sub_matches);
//             },
//             None => { todo!("start tui") },
//             _ => unreachable!("should be unreachable")
//         }
//         // if let Some(_matches) = matches.subcommand_matches("provider_api") {
//         //     // println!("provider_api command");
//         // }
//
//         Ok(())
//     }
// }
