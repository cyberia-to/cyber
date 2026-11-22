//! Where the spell lives. The same words are the same neuron on every body
//! (cyb keeps them in `~/cyb/spell`, older installs in `~/cyb/mnemonic`);
//! `cyber` reads `--spell FILE`, else `$CYBER_SPELL`, else `$CYBER_HOME/spell`
//! (default `~/.cyber/spell`), else cyb's file — so one identity needs one
//! backup.

use std::path::{Path, PathBuf};

use crate::Result;

fn home() -> Result<PathBuf> {
    std::env::var_os("HOME").map(PathBuf::from).ok_or_else(|| "HOME is required".into())
}

pub fn path(flag: Option<&str>, home_flag: Option<&str>) -> Result<PathBuf> {
    if let Some(p) = flag.map(str::to_string).or_else(|| std::env::var("CYBER_SPELL").ok()) {
        return Ok(PathBuf::from(p));
    }
    let own = match home_flag.map(PathBuf::from).or_else(|| std::env::var_os("CYBER_HOME").map(PathBuf::from)) {
        Some(h) => h.join("spell"),
        None => home()?.join(".cyber").join("spell"),
    };
    if own.is_file() {
        return Ok(own);
    }
    let cyb = home()?.join("cyb");
    for name in ["spell", "mnemonic"] {
        let p = cyb.join(name);
        if p.is_file() {
            return Ok(p);
        }
    }
    Ok(own)
}

pub fn read(path: &Path) -> Result<String> {
    let words = std::fs::read_to_string(path)
        .map_err(|e| format!("{}: {e}; mint one with `cyber neuron new` or point --spell at your words", path.display()))?;
    let words = words.trim();
    if words.is_empty() {
        return Err(format!("{}: empty spell", path.display()));
    }
    Ok(words.to_string())
}

/// Write a fresh spell, refusing to overwrite: the file is the owner.
pub fn write_new(path: &Path, words: &str) -> Result<()> {
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    }
    let mut options = std::fs::OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    use std::io::Write;
    let mut file = options
        .open(path)
        .map_err(|e| format!("{}: {e} (an existing spell is never overwritten)", path.display()))?;
    file.write_all(words.as_bytes()).and_then(|_| file.sync_all()).map_err(|e| e.to_string())
}
