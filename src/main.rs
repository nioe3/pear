pub mod services;
pub mod tui;
mod cli;
mod util;


#[tokio::main]
async fn main() {
    let res = cli::app::App::new().run().await;
}
