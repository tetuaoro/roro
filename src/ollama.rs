use ollama_rs::{
    Ollama,
    generation::chat::{ChatMessage, request::ChatMessageRequest},
    models::ModelOptions,
};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tokio_stream::StreamExt;

pub struct OllamaHandler {
    ollama: Ollama,
    history: Arc<Mutex<Vec<ChatMessage>>>,
    model_name: String,
    model_options: ModelOptions,
    receiver: mpsc::Receiver<String>,
    sender: mpsc::Sender<ChatMessage>,
}

impl OllamaHandler {
    pub fn new(
        ollama: Ollama,
        history: Arc<Mutex<Vec<ChatMessage>>>,
        model_name: String,
        model_options: ModelOptions,
        receiver: mpsc::Receiver<String>,
        sender: mpsc::Sender<ChatMessage>,
    ) -> Self {
        Self {
            ollama,
            history,
            model_name,
            model_options,
            receiver,
            sender,
        }
    }

    pub async fn run(mut self) {
        while let Some(input) = self.receiver.recv().await {
            let history = self.history.clone();
            let ollama = self.ollama.clone();
            let model_name = self.model_name.clone();
            let model_options = self.model_options.clone();
            let sender = self.sender.clone();

            tokio::spawn(async move {
                let mut stream = ollama
                    .send_chat_messages_with_history_stream(
                        history,
                        ChatMessageRequest::new(model_name, vec![ChatMessage::user(input)])
                            .tools(vec![])
                            .options(model_options),
                    )
                    .await
                    .unwrap();

                while let Some(res) = stream.next().await {
                    if let Ok(response) = res {
                        if !response.done {
                            let _ = sender.send(response.message).await;
                        }
                    }
                }
            });
        }
    }
}
