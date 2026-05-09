use clap::Parser;
use url::Url;

#[derive(Parser)]
#[command(version, about, author)]
pub enum RoroCli {
    Start(Start),
}

#[derive(clap::Args, Clone)]
pub struct Start {
    /// Ollama provider URL
    #[arg(short, long, env, default_value = "http://127.0.0.1:11434")]
    pub provider_base_url: Url,

    /// Model name
    #[arg(short, long)]
    pub model_name: String,

    /// Number of threads
    #[arg(short, long, default_value = "2")]
    pub num_thread: u32,
}

impl RoroCli {
    pub fn build() -> Self {
        RoroCli::parse()
    }
}
