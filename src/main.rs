mod cli;

use cli::RoroCli;

use futures_util::TryStreamExt;

use ollama_rs::{
    Ollama,
    generation::chat::{ChatMessage, request::ChatMessageRequest},
    models::ModelOptions,
};
use std::sync::{Arc, Mutex};

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, stdout};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let RoroCli::Start(cli_start) = RoroCli::build();

    let ollama = Ollama::from_url(cli_start.provider_base_url);

    let history = Arc::new(Mutex::new(vec![]));
    let mut stdout = stdout();

    let mut commands = std::pin::pin!({
        let stdin_reader = tokio::io::BufReader::new(tokio::io::stdin());
        let line_stream = tokio_stream::wrappers::LinesStream::new(stdin_reader.lines());
        line_stream.try_take_while(|x| std::future::ready(Ok(!x.eq_ignore_ascii_case("exit"))))
    });

    loop {
        let model_name = cli_start.model_name.clone();
        let model_options = ModelOptions::default().num_thread(cli_start.num_thread);

        stdout.write_all(b"\n> ").await?;
        stdout.flush().await?;

        let input = match commands.next().await {
            Some(Ok(x)) => x,
            Some(Err(e)) => return Err(e.into()),
            None => break,
        };

        let mut stream = ollama
            .send_chat_messages_with_history_stream(
                history.clone(),
                ChatMessageRequest::new(model_name, vec![ChatMessage::user(input)])
                    .tools(vec![])
                    .options(model_options),
            )
            .await?;

        while let Some(res) = stream.next().await {
            let res = res.map_err(|_| anyhow::anyhow!("Error during stream"))?;
            if let Some(thinking) = res.message.thinking {
                stdout.write_all(thinking.as_bytes()).await?;
            }
            stdout.write_all(res.message.content.as_bytes()).await?;
            stdout.flush().await?;
        }
    }

    dbg!(&history.lock().unwrap());

    Ok(())
}
