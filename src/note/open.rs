use crate::fs::paths;
use std::path::{Path, PathBuf};
use std::process::Command;
use anyhow::{Result, bail};
use walkdir::WalkDir;
use std::fs;
use chrono::Local;
use serde_yaml;
// use crate::note::model::Note;
use crate::note::read::read_note_file;

/// Get file's last modified time
fn get_file_modified(path: &Path) -> Result<chrono::DateTime<Local>> {
    let metadata = fs::metadata(path)?;
    let modified_time = metadata.modified()?;
    Ok(modified_time.into())
}

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

 // --- 1. Read existing note ---
    let mut note = read_note_file(path_to_open)?.meta;

    // --- 2. Sync `updated` if file changed externally ---
    let mtime = get_file_modified(path_to_open)?;
    if mtime > note.updated {
        note.updated = mtime;

        // Rewrite YAML front matter without touching body
        let yaml = serde_yaml::to_string(&note)?;
        let content = fs::read_to_string(path_to_open)?;
        let parts: Vec<&str> = content.splitn(3, "---").collect();
        let body = parts.get(2).unwrap_or(&"");

        let new_content = format!("---\n{}---{}", yaml, body);
        fs::write(path_to_open, new_content)?;
    }

    
    let editor = editor
        .or_else(|| std::env::var("EDITOR").ok())
        .unwrap_or_else(|| "vim".to_string());

    Command::new(editor)
        .arg(path_to_open)
        .status()?;

    Ok(())
}
