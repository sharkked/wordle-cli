use worcl::{self, Game};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let game = Game::new().await?;

    worcl::cli::run(game)?;

    Ok(())
}
