use crate::{storage::home_dir, Result};
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};

pub struct Cache {
    dir: String,
    allowed_words: String,
    valid_words: String,
}

impl Cache {
    pub async fn init() -> Result<Cache> {
        let cache_dir = format!("{}/.cache/wordle-cli", home_dir());
        let allowed_words = format!("{}/allowed-words", cache_dir);
        let valid_words = format!("{}/valid-words", cache_dir);
        let cache = Cache {
            dir: cache_dir,
            allowed_words,
            valid_words,
        };

        cache.rebuild_all(false).await?;

        Ok(cache)
    }

    async fn rebuild_all(&self, forced: bool) -> Result<()> {
        if forced || !fs::metadata(&self.allowed_words).is_ok() {
            self.rebuild_allowed().await?;
        }

        if forced || !fs::metadata(&self.valid_words).is_ok() {
            self.rebuild_valid().await?;
        }

        Ok(())
    }

    async fn rebuild_allowed(&self) -> Result<()> {
        fs::create_dir_all(&self.dir)?;
        let mut file = File::create(&self.allowed_words)?;

        let url =
            "https://raw.githubusercontent.com/Roy-Orbison/wordle-guesses-answers/main/guesses.txt";
        let words_list = reqwest::get(url).await?.text().await?;

        file.write_all(&words_list.as_bytes())?;

        Ok(())
    }

    async fn rebuild_valid(&self) -> Result<()> {
        fs::create_dir_all(&self.dir)?;
        let mut file = File::create(&self.valid_words)?;

        let url =
            "https://raw.githubusercontent.com/Roy-Orbison/wordle-guesses-answers/main/answers.txt";
        let words_list = reqwest::get(url).await?.text().await?;

        file.write_all(&words_list.as_bytes())?;

        Ok(())
    }

    pub fn find_word(&self, word: &String) -> Result<bool> {
        let word = word.to_lowercase();
        let file = File::open(&self.allowed_words)?;
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if line?.trim() == word {
                return Ok(true);
            }
        }

        Ok(false)
    }
}
