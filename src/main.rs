use wordle::{Application, Game, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let game = Game::new().await?;

    Application::init(game).await?.run()?;

    Ok(())
}
