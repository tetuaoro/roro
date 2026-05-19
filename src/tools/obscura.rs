use super::BashExecutor;
use super::Result;
use ollama_rs::generation::tools::Tool;
use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub enum ObscuraSubcommand {
    #[schemars(description = "Fetch the content of a single URL. Use this to retrieve HTML, text, or links from a webpage.")]
    Fetch,
    #[schemars(description = "Scrape multiple URLs concurrently. Use this for bulk web scraping tasks.")]
    Scrape,
    #[schemars(description = "Print the help message or the help of the given subcommand(s).")]
    Help,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ObscuraParams {
    #[schemars(description = "The Obscura subcommand to execute (fetch, scrape, help).")]
    pub subcommand: ObscuraSubcommand,
    #[schemars(
        description = "Arguments for the subcommand. For 'fetch', provide the URL. For 'scrape', provide one or more URLs. For 'serve', use flags like '--port' or '--stealth', more with --help"
    )]
    pub args: Vec<String>,
    #[schemars(description = "Optional timeout in seconds (default: 30).")]
    pub timeout_secs: Option<u64>,
}

pub struct ObscuraTool;

impl Default for ObscuraTool {
    fn default() -> Self {
        Self::new()
    }
}

impl ObscuraTool {
    pub fn new() -> Self {
        ObscuraTool
    }

    pub async fn execute_obscura(&self, subcommand: ObscuraSubcommand, args: Vec<String>, timeout_secs: Option<u64>) -> Result<String> {
        let mut command = match subcommand {
            ObscuraSubcommand::Fetch => "obscura fetch".to_string(),
            ObscuraSubcommand::Scrape => "obscura scrape".to_string(),
            ObscuraSubcommand::Help => "obscura help".to_string(),
        };

        for arg in args {
            command.push_str(&format!(" {}", arg));
        }

        let bash_executor = BashExecutor::new();
        bash_executor.execute_command(command, timeout_secs).await
    }
}

impl Tool for ObscuraTool {
    type Params = ObscuraParams;

    fn name() -> &'static str {
        "Obscura"
    }

    fn description() -> &'static str {
        r#"A lightweight headless browser for web navigation or scraping.
        Use 'fetch' to retrieve a single URL,'scrape' for bulk scraping, or 'help' for usage instructions."#
    }

    async fn call(&mut self, params: ObscuraParams) -> Result<String> {
        self.execute_obscura(params.subcommand, params.args, params.timeout_secs).await
    }
}
