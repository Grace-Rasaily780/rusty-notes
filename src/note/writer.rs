use crate::note::model::Note;
use chrono::Local;
use std::fs;
use std::path::Path;

pub fn write_note(path: &Path, title: String, tags: Vec<String>) -> anyhow::Result<()> {
    fs::create_dir_all(path.parent().unwrap())?;

    let now = Local::now();
    let note = Note {
        id: now.format("%Y%m%d%H%M%S").to_string(),
        title: title.clone(),
        tags,
        created: now,
        updated: now,
    };

    let yaml = serde_yaml::to_string(&note)?;
    let content = format!(
        "---\n{}---\n\n",
        yaml,
    );

    fs::write(path, content)?;
    Ok(())
}
