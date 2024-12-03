use anyhow::Result;
use eval_stack::engine::runtime::listen_for_submissions;

#[tokio::main]
async fn main() -> Result<()> {
    listen_for_submissions().await?;
    Ok(())
}
