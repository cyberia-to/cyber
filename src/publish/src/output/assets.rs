use crate::scanner::DiscoveredFiles;
use anyhow::Result;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

/// Copy discovered assets to output directory.
pub fn copy_assets(discovered: &DiscoveredFiles, output_dir: &Path) -> Result<()> {
    if discovered.assets.is_empty() {
        return Ok(());
    }

    let assets_output = output_dir.join("assets");
    fs::create_dir_all(&assets_output)?;

    for asset in &discovered.assets {
        let dest = assets_output.join(&asset.name);
        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::copy(&asset.path, &dest)?;
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
