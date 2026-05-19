use crate::tools;
use ollama_rs::{Ollama as OllamaRs, coordinator::Coordinator, generation::chat::ChatMessage, history::ChatHistory, models::ModelOptions};
use std::{
    borrow::Cow,
    sync::{Arc, Mutex},
};
use tokio::sync::mpsc;

struct CustomHistory {
    messages: Arc<Mutex<Vec<ChatMessage>>>,
}

impl ChatHistory for CustomHistory {
    fn push(&mut self, message: ChatMessage) {
        self.messages.lock().unwrap().push(message);
    }

    fn messages(&self) -> Cow<'_, [ChatMessage]> {
        let messages = self.messages.lock().unwrap();
        Cow::Owned(messages.clone())
    }
}

pub struct Ollama {
    receiver: mpsc::Receiver<String>,
    sender: mpsc::Sender<ChatMessage>,
    coordinator: Coordinator<CustomHistory>,
    history: Arc<Mutex<Vec<ChatMessage>>>,
}

impl Ollama {
    pub fn new(
        ollama: OllamaRs,
        model_name: String,
        model_options: ModelOptions,
        receiver: mpsc::Receiver<String>,
        sender: mpsc::Sender<ChatMessage>,
    ) -> Self {
        let history = Arc::new(Mutex::new(Vec::new()));
        let custom_history = CustomHistory { messages: history.clone() };

        let coordinator = Coordinator::new(ollama, model_name, custom_history)
            .options(model_options)
            .add_tool(tools::ObscuraTool::new())
            .add_tool(tools::BashExecutor::new());

        Self {
            sender,
            receiver,
            coordinator,
            history,
        }
    }

    pub async fn run(mut self) {
        while let Some(input) = self.receiver.recv().await {
            if let Ok(response) = self.coordinator.chat(vec![ChatMessage::user(input)]).await {
                _ = self.sender.send(response.message).await;
            }
        }
    }

    pub fn history(&self) -> Arc<Mutex<Vec<ChatMessage>>> {
        self.history.clone()
    }
}
