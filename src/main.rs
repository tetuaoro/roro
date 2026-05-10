mod cli;
mod tui;

use std::sync::{Arc, Mutex};

use ollama_rs::{Ollama, models::ModelOptions};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli::RoroCli::Start(cli_start) = cli::RoroCli::build();

    let ollama = Ollama::from_url(cli_start.provider_base_url);
    let history = Arc::new(Mutex::new(vec![]));
    let model_options = ModelOptions::default().num_thread(cli_start.num_thread);

    tui::run(ollama, history, cli_start.model_name, model_options).map_err(|e| anyhow::anyhow!(e.to_string()))?;

    Ok(())
}
