use crate::{storage::home_dir, Game, Result};
use std::{fs, path::Path};

pub struct LocalShare {
    dir: String,
}

impl LocalShare {
    pub fn init() -> Result<Self> {
        let local_share_dir = format!("{}/.local/share/wordle-cli", home_dir());

        Ok(Self {
            dir: local_share_dir,
        })
    }

    pub fn save(&self, file: String, game: &Game) -> Result<()> {
        let path = format!("{}/{}", &self.dir, &file);
        let path = Path::new(&path);
        let prefix = path.parent().unwrap();
        fs::create_dir_all(prefix)?;

        Ok(())
    }
}
