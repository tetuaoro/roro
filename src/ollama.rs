use ollama_rs::{Ollama as OllamaRs, coordinator::Coordinator, generation::chat::ChatMessage, models::ModelOptions};
use tokio::sync::mpsc;

use crate::tools;

pub struct Ollama {
    receiver: mpsc::Receiver<String>,
    sender: mpsc::Sender<ChatMessage>,
    coordinator: Coordinator<Vec<ChatMessage>>,
}

impl Ollama {
    pub fn new(
        ollama: OllamaRs,
        model_name: String,
        model_options: ModelOptions,
        receiver: mpsc::Receiver<String>,
        sender: mpsc::Sender<ChatMessage>,
    ) -> Self {
        let coordinator = Coordinator::new(ollama, model_name, vec![])
            .options(model_options)
            .add_tool(tools::BashExecutor::new());

        Self {
            sender,
            receiver,
            coordinator,
        }
    }

    pub async fn run(mut self) {
        while let Some(input) = self.receiver.recv().await {
            if let Ok(response) = self.coordinator.chat(vec![ChatMessage::user(input)]).await {
                _ = self.sender.send(response.message).await;
            }
        }
    }
}
