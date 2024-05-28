use chrono;
use serde::Deserialize;
use thousands::Separable;

#[derive(Deserialize)]
pub struct WordleJson {
    pub id: i32,
    pub solution: String,
    pub print_date: String,
    pub days_since_launch: i32,
    pub editor: String,
}

impl WordleJson {
    pub async fn fetch() -> Result<WordleJson, reqwest::Error> {
        let date = chrono::offset::Local::now().date_naive();
        let url = format!("https://www.nytimes.com/svc/wordle/v2/{}.json", date);
        Ok(reqwest::get(url).await?.json::<WordleJson>().await?)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum CharState {
    #[default]
    Default,
    Correct(char),
    WrongPlace(char),
    Absent(char),
}

pub struct Game {
    pub id: Option<i32>,
    pub title: String,
    pub solution: String,
    pub guesses: Vec<Vec<CharState>>,
    pub guessed_chars: [CharState; 26],
    pub result: Option<i32>,
    pub hardmode: bool,
}

fn char_to_idx(c: char) -> usize {
    c.to_ascii_lowercase() as usize - 97
}

impl Game {
    pub async fn new() -> Result<Self, reqwest::Error> {
        let puzzle = WordleJson::fetch().await?;
        let solution = puzzle.solution.to_uppercase();

        Ok(Game {
            id: Some(puzzle.days_since_launch),
            title: format!("Wordle {}", puzzle.days_since_launch.separate_with_commas()),
            solution,
            guesses: vec![],
            guessed_chars: [CharState::Default; 26],
            result: None,
            hardmode: false,
        })
    }

    pub fn check_guess(&mut self, guess: String) -> Option<Vec<CharState>> {
        if !self.result.is_none() || self.guesses.len() >= 6 {
            return None;
        }

        // validate guess
        if guess.len() != 5 {
            return None;
        }

        if guess == self.solution {
            self.result = Some(self.guesses.len() as i32 + 1)
        } else if self.guesses.len() == 5 {
            self.result = Some(-1)
        }

        // count correct characters
        let mut guess_state = vec![CharState::Default; 5];
        let mut remaining_chars = [0; 26];
        for (i, c) in self.solution.char_indices() {
            if guess.chars().nth(i).unwrap() == c {
                guess_state[i] = CharState::Correct(c);
            } else {
                remaining_chars[char_to_idx(c)] += 1;
            }
        }

        // find misplaced characters
        for (i, c) in guess.char_indices() {
            if !matches!(guess_state[i], CharState::Correct(_)) {
                let idx = char_to_idx(c);
                guess_state[i] = match remaining_chars[idx] > 0 {
                    true => CharState::WrongPlace(c),
                    false => CharState::Absent(c),
                };
                remaining_chars[idx] -= 1;
            }
        }

        self.guesses.push(guess_state.clone());
        Some(guess_state)
    }

    pub fn fmt_share(&self) -> String {
        let mut out_str = String::new();

        match self.result {
            Some(-1) => out_str.push_str(&format!("{} X/6\n", self.title)),
            Some(n) => out_str.push_str(&format!("{} {}/6\n", self.title, n)),
            _ => return String::new(),
        }

        for guess in &self.guesses {
            for c in guess {
                out_str.push(match c {
                    CharState::Correct(_) => '🟩',
                    CharState::WrongPlace(_) => '🟨',
                    CharState::Absent(_) => '⬛',
                    CharState::Default => continue,
                });
            }
            out_str.push('\n');
        }
        out_str.trim().into()
    }
}
