pub mod setup;

use clap::Subcommand;
use setup::ProviderSetupCommands;

#[derive(Subcommand)]
pub enum ProviderCommands {

    /// setup a provider_api
    #[command(subcommand)]
    Setup(ProviderSetupCommands)
}









// pub trait SetupCommandManager {
//     fn read_provider_name(&self) -> &'static str;
//
//     fn setup_args(&self) -> Vec<Arg>;
//     fn setup_matches(&self, matches: &ArgMatches) -> std::io::Result<()>;
// }
// pub struct ProviderCommandManager {}
//
// impl ProviderCommandManager {
//     pub fn commands(&self) -> Command {
//         let mut setup_command = Command::new("setup")
//             .subcommand_required(true)
//             .about("Setup a model provider_api");
//
//         let setup_command_managers: Vec<Box<dyn SetupCommandManager>> = vec![
//             Box::new(OllamaCommandManager{})
//         ];
//
//         for manager in &setup_command_managers {
//             setup_command = setup_command.subcommand(
//                 Command::new(manager.read_provider_name())
//                     .args(manager.setup_args())
//             )
//         }
//
//         // dbg!(&setup_command.get_name());
//
//         let command = Command::new("provider_api")
//             .about("Provider commands")
//             .subcommand(
//                 setup_command
//             );
//
//         // dbg!(&command);
//
//         command
//     }
// }
//
//
//
// pub fn run_matches(matches: &ArgMatches) {
//     match matches.subcommand() {
//         Some(("setup", sub_matches)) => {
//             match sub_matches.subcommand() {
//                 Some(("ollama", sub_matches)) => {
//
//                 },
//                 Some(("anthropic", sub_matches)) => {
//
//                 },
//                 _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`")
//             }
//         },
//         _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`")
//     }
//     if let Some(matches) = matches.subcommand_matches("setup") {
//         if let Some(matches) = matches.subcommand_matches("ollama") {
//             ollama::OllamaCommandManager{}.setup_matches(matches).expect("error matching ollama command");
//         }else if let Some(matches) = matches.subcommand_matches("anthropic") {
//             todo!("Add anthropic provider_api")
//         }
//     }
// }