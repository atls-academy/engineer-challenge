use rust_kratos::application::bootstrap;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    bootstrap::run().await
}
