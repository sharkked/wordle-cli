use wordle::{self, Game};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let game = Game::new().await?;

    wordle::cli::run(game)?;

    Ok(())
}
