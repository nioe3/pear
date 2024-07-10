use clap::Subcommand;

#[derive(Subcommand, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum ProviderSetupCommands {
    /// configure ollama
    Ollama {
        /// set host address of ollama
        #[arg(short = 'a', long, default_value = "http://127.0.0.1:11434")]
        host: Option<String>,
        /// sets what madel to use
        #[arg(short, long, default_value = "codellama")]
        model: Option<String>
    },
    /// configure anthropic
    Anthropic {
        /// your anthropic api key
        #[arg(short, long, required = true)]
        key: String,

        /// sets what model to use
        #[arg(short, long, required = true)]
        model: String
    }
}