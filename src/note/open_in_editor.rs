use std::process::Command;
use std::path::Path;

pub fn open_in_editor(path: &Path, editor: Option<String>) -> anyhow::Result<()> {
    let editor = editor
        .or_else(|| std::env::var("EDITOR").ok())
        .unwrap_or_else(|| "vim".to_string());

    Command::new(editor)
        .arg(path)
        .status()?;

    Ok(())
}
