use wordle::{Application, Game};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let game = Game::new().await?;

    Application::init(game).await?.run()?;

    Ok(())
}
