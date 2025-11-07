use rust_llm_proxy::app::app;

#[tokio::main]
async fn main() {
    if let Err(e) = app().await {
        eprintln!("Application failed to start: {}", e);
        std::process::exit(1);
    }
}
