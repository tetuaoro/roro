mod cli;
mod ollama;
mod tui;

use ollama_rs::{Ollama, models::ModelOptions};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let cli::RoroCli::Start(cli_start) = cli::RoroCli::build();

    let ollama = Ollama::from_url(cli_start.provider_base_url);
    let history = Arc::new(Mutex::new(vec![]));
    let model_options = ModelOptions::default().num_thread(cli_start.num_thread);

    let (tui_sender, ollama_receiver) = mpsc::channel(32);
    let (ollama_sender, tui_receiver) = mpsc::channel(32);

    let ollama_handler = ollama::OllamaHandler::new(
        ollama,
        history.clone(),
        cli_start.model_name,
        model_options,
        ollama_receiver,
        ollama_sender,
    );

    let app = tui::TuiApp::new(tui_sender, tui_receiver);
    tokio::spawn(ollama_handler.run());
    ratatui::run(|terminal| app.run(terminal))?;

    #[cfg(debug_assertions)]
    let _ = dbg!(history);

    Ok(())
}
