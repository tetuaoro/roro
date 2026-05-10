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

    let ollama_handler = ollama::OllamaHandler::new(ollama, history.clone(), cli_start.model_name, model_options);

    let (sender, receiver) = mpsc::channel(32);

    let app = tui::TuiApp::new(ollama_handler, sender, receiver);
    ratatui::run(|terminal| app.run(terminal))?;

    #[cfg(debug_assertions)]
    let _ = dbg!(history);

    Ok(())
}
