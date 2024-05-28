use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};

pub fn home_dir() -> String {
    std::env::var_os("HOME").unwrap().into_string().unwrap()
}

pub struct Cache {
    dir: String,
    allowed_words: String,
}

impl Cache {
    pub async fn init() -> Result<Cache, Box<dyn std::error::Error + Send + Sync>> {
        let cache_dir = format!("{}/.cache/wordle-cli", home_dir());
        let allowed_words = format!("{}/allowed-words", cache_dir);
        let cache = Cache {
            dir: cache_dir,
            allowed_words,
        };

        if !fs::metadata(&cache.allowed_words).is_ok() {
            cache.rebuild().await?;
        }

        Ok(cache)
    }

    pub async fn rebuild(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        fs::create_dir_all(&self.dir)?;
        let mut file = File::create(&self.allowed_words)?;

        let url =
            "https://raw.githubusercontent.com/Roy-Orbison/wordle-guesses-answers/main/guesses.txt";
        let words_list = reqwest::get(url).await?.text().await?;

        file.write_all(&words_list.as_bytes())?;

        Ok(())
    }

    pub fn find_word(&self, word: &String) -> Result<bool, Box<dyn std::error::Error>> {
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
