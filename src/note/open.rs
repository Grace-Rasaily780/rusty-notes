use crate::fs::paths;
use std::path::{PathBuf};
use std::process::Command;
use anyhow::{Result, bail};
use walkdir::WalkDir;

/// Open a note by query string
pub fn open_note(query: String, notes_dir: &str, editor: Option<String>) -> Result<()> {
    let notes_dir = paths::notes_dir(notes_dir);

    let mut matches: Vec<PathBuf> = Vec::new();
    for entry in WalkDir::new(&notes_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.extension().map(|s| s == "note").unwrap_or(false) {
            let filename = path.file_stem().unwrap().to_string_lossy();
            if filename == query {
                matches.push(path.to_path_buf());
            }
        }
    }

    if matches.is_empty() {
        bail!("No note matching '{}' found in {:?}", query, notes_dir);
    }

    let path_to_open = &matches[0];

    let editor = editor
        .or_else(|| std::env::var("EDITOR").ok())
        .unwrap_or_else(|| "vim".to_string());

    Command::new(editor)
        .arg(path_to_open)
        .status()?;

    Ok(())
}
