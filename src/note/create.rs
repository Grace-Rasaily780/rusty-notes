use crate::fs::paths;
use crate::note::open_in_editor;
use crate::note::writer::write_note;
use std::path::PathBuf;

pub fn create(
    title: Option<String>,
    tags: Vec<String>,
    dir: &str,
    editor: Option<String>,
) -> anyhow::Result<()> {
    let title = title.unwrap_or_else(|| "Untitled".into());
    let dir = paths::inbox_dir(dir);

    let filename = title.replace(" ", "_").to_lowercase();
    let path = PathBuf::from(dir).join(format!("{}.note", filename));

    write_note(&path, title, tags)?;

    open_in_editor::open_in_editor(&path, editor)?;
    Ok(())
}
