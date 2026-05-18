use ollama_rs::generation::tools::Tool;
use schemars::JsonSchema;
use serde::Deserialize;
use std::io::{Error, ErrorKind};
use std::process::Command;
use std::time::Duration;
use tokio::time::timeout;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Sync + Send>>;

#[derive(Deserialize, JsonSchema)]
pub struct BashCommandParams {
    #[schemars(description = "The Bash command to execute (e.g., 'ls -la').")]
    pub command: String,
    #[schemars(description = "Optional timeout in seconds (default: 30).")]
    pub timeout_secs: Option<u64>,
}

pub struct BashExecutor;

impl Default for BashExecutor {
    fn default() -> Self {
        Self::new()
    }
}

impl BashExecutor {
    pub fn new() -> Self {
        BashExecutor
    }

    pub async fn execute_command(&self, command: String, timeout_secs: Option<u64>) -> Result<String> {
        let timeout_duration = Duration::from_secs(timeout_secs.unwrap_or(30));
        let cmd = Command::new("sh").arg("-c").arg(&command).output();
        let output = timeout(timeout_duration, async { cmd }).await;

        match output {
            Ok(Ok(output)) => {
                if output.status.success() {
                    Ok(String::from_utf8(output.stdout)?)
                } else {
                    let error_msg = String::from_utf8(output.stderr).unwrap_or_else(|_| "Command failed with non-UTF8 error".to_string());
                    Err(Box::new(Error::new(ErrorKind::Other, error_msg)))
                }
            }
            Ok(Err(e)) => Err(Box::new(e)),
            Err(_) => Err(Box::new(Error::new(
                ErrorKind::TimedOut,
                format!("Command timed out after {} seconds", timeout_duration.as_secs()),
            ))),
        }
    }
}

impl Tool for BashExecutor {
    type Params = BashCommandParams;

    fn name() -> &'static str {
        "Bash"
    }

    fn description() -> &'static str {
        "Executes a bash command and returns its stdout or stderr."
    }

    async fn call(&mut self, params: BashCommandParams) -> Result<String> {
        self.execute_command(params.command, params.timeout_secs).await
    }
}
