pub mod game;
pub mod storage;

use game::CharState;
pub use game::Game;
use std::io::{self, Write};
use storage::Cache;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub struct Application {
    game: Game,
    cache: storage::Cache,
}

impl Application {
    pub async fn init(game: Game) -> Result<Self> {
        let cache = Cache::init().await?;
        Ok(Self { game, cache })
    }

    pub fn run(&mut self) -> Result<()> {
        println!("{}", &self.game.title);

        let mut input = String::new();

        while self.game.result.is_none() {
            print!("[{}/6] ", &self.game.guesses.len() + 1);
            let _ = io::stdout().flush();

            input.clear();
            io::stdin().read_line(&mut input).unwrap();

            if let Some(result) = self.game.check_guess(&input, &self.cache) {
                println!("{}", pretty_string(result, '■', true));
            };
        }

        println!("\n======");

        println!("{}", self.game.fmt_share());

        Ok(())
    }
}

fn pretty_string(guess: Vec<CharState>, box_char: char, show_text: bool) -> String {
    let mut guess_text = String::new();
    let mut guess_boxes = String::new();
    let mut prev_state = CharState::Absent(' ');
    let mut curr_ansi_color = "\x1b[0m";
    let mut curr_char;
    for &state in guess.iter() {
        (curr_ansi_color, curr_char) = match state {
            CharState::WrongPlace(c) => ("\x1b[0;33m", c),
            CharState::Correct(c) => ("\x1b[0;32m", c),
            CharState::Absent(c) => ("\x1b[0m", c),
            CharState::Default => continue,
        };

        if prev_state != state {
            guess_text.push_str(&format!("{}{}", curr_ansi_color, curr_char));
            guess_boxes.push_str(&format!("{}{}", curr_ansi_color, box_char));
        } else {
            guess_text.push_str(&format!("{}", curr_char));
            guess_boxes.push_str(&format!("{}", box_char));
        }
        prev_state = state;
    }
    if curr_ansi_color != "\x1b[0m" {
        guess_text.push_str("\x1b[0m");
        guess_boxes.push_str("\x1b[0m");
    }

    match show_text {
        true => format!("{} {}", guess_text, guess_boxes),
        false => guess_boxes,
    }
}
