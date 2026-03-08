use rust_kratos::application::boostrap;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    boostrap::run().await
}
