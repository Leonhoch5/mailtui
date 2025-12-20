use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SavedToken {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_at_unix: Option<i64>,
}

fn config_dir() -> PathBuf {
    if let Some(dir) = dirs::config_dir() {
        dir.join("mailtui")
    } else {
        let mut p = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        p.push(".config/mailtui");
        p
    }
}

fn token_file() -> PathBuf {
    let mut d = config_dir();
    d.push("token.json");
    d
}

pub fn save_token(token: &SavedToken) -> io::Result<()> {
    let dir = config_dir();
    fs::create_dir_all(&dir)?;
    let tmp = token_file().with_extension("tmp");
    let data = serde_json::to_string_pretty(token).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    let mut f = fs::File::create(&tmp)?;
    f.write_all(data.as_bytes())?;
    f.flush()?;
    fs::rename(tmp, token_file())?;
    Ok(())
}

pub fn load_token() -> io::Result<SavedToken> {
    let p = token_file();
    let s = fs::read_to_string(p)?;
    let t: SavedToken = serde_json::from_str(&s).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    Ok(t)
}
