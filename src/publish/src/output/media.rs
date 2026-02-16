use crate::scanner::DiscoveredFiles;
use anyhow::Result;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

/// Copy discovered media to output directory.
pub fn copy_media(discovered: &DiscoveredFiles, output_dir: &Path) -> Result<()> {
    if discovered.media.is_empty() {
        return Ok(());
    }

    let media_output = output_dir.join("media");
    fs::create_dir_all(&media_output)?;

    for file in &discovered.media {
        let dest = media_output.join(&file.name);
        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::copy(&file.path, &dest)?;
    }

    Ok(())
}

/// Recursively copy a directory.
pub fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<()> {
    for entry in WalkDir::new(src).into_iter().filter_map(|e| e.ok()) {
        let relative = entry.path().strip_prefix(src)?;
        let target = dst.join(relative);

        if entry.file_type().is_dir() {
            fs::create_dir_all(&target)?;
        } else {
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(entry.path(), &target)?;
        }
    }
    Ok(())
}
