use crate::game::{CharState, Game};
use std::io::{self, Write};

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

pub fn run(mut game: Game) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("{}", &game.title);

    let mut input = String::new();

    while game.result.is_none() {
        print!("[{}/6] ", game.guesses.len() + 1);
        let _ = io::stdout().flush();

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let guess = input.trim().to_uppercase();

        if !guess.bytes().all(|b| matches!(b, b'A'..=b'Z')) {
            continue;
        }

        if let Some(result) = game.check_guess(guess) {
            println!("{}", pretty_string(result, '■', true));
        };
    }

    println!("\n======");

    println!("{}", game.fmt_share());

    Ok(())
}
