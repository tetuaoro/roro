mod cli;
mod ollama;
mod tools;
mod tui;

use ollama_rs::{Ollama, models::ModelOptions};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let cli::Cli::Start(cli_start) = cli::Cli::build();

    let ollama = Ollama::from_url(cli_start.provider_base_url);
    let model_options = ModelOptions::default().num_thread(cli_start.num_thread);

    let (tui_sender, ollama_receiver) = mpsc::channel(32);
    let (ollama_sender, tui_receiver) = mpsc::channel(32);

    let ollama = ollama::Ollama::new(
        ollama,
        cli_start.model_name,
        model_options,
        ollama_receiver,
        ollama_sender,
    );

    let app = tui::App::new(tui_sender, tui_receiver);
    tokio::spawn(ollama.run());
    ratatui::run(|terminal| app.run(terminal))?;

    Ok(())
}
