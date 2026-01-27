use anyhow::{anyhow, Result, bail};
use std::fs;
use std::path::{Path, PathBuf};
use ansi_term::Colour::{Cyan, Fixed};
use ansi_term::Style;
use chrono::Local;
use chrono_humanize::HumanTime;
use textwrap::fill;
use walkdir::WalkDir;
use crate::fs::paths;
use crate::note::model::Note;

pub struct ParsedNote {
    pub meta: crate::note::model::Note,
    pub body: String,
}

/// Sync the note's updated field to the file's last modified time
pub fn sync_updated(path: &Path, note: &mut Note) -> Result<()> {
    let metadata = fs::metadata(path)?;
    let mtime = metadata.modified()?;
    let mtime: chrono::DateTime<Local> = mtime.into();

    if mtime > note.updated {
        note.updated = mtime;

        // Rewrite YAML front matter without touching body
        let yaml = serde_yaml::to_string(note)?;
        let content = fs::read_to_string(path)?;
        let parts: Vec<&str> = content.splitn(3, "---").collect();
        let body = parts.get(2).unwrap_or(&"");

        let new_content = format!("---\n{}---{}", yaml, body);
        fs::write(path, new_content)?;
    }

    Ok(())
}

pub fn read_note_file(path: &Path) -> Result<ParsedNote> {
    let content = fs::read_to_string(path)?;

    let mut parts = content.splitn(3, "---");

    parts.next(); // discard before first ---
    let yaml = parts
        .next()
        .ok_or_else(|| anyhow!("Missing YAML front matter"))?;
    let body = parts
        .next()
        .ok_or_else(|| anyhow!("Missing note body"))?;

    let mut meta = serde_yaml::from_str(yaml)?;

    sync_updated(path, &mut meta)?;
    Ok(ParsedNote {
        meta,
        body: body.trim().to_string(),
    })
}

pub fn read_note(query: String, path: &str) -> Result<ParsedNote> {
    let notes_dir = paths::notes_dir(path);

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

    let content = fs::read_to_string(path_to_open)?;

    let mut parts = content.splitn(3, "---");

    parts.next(); // discard before first ---
    let yaml = parts
        .next()
        .ok_or_else(|| anyhow!("Missing YAML front matter"))?;
    let body = parts
        .next()
        .ok_or_else(|| anyhow!("Missing note body"))?;

    let meta = serde_yaml::from_str(yaml)?;

    Ok(ParsedNote {
        meta,
        body: body.trim().to_string(),
    })
}

pub fn render_note(note: ParsedNote) {
    let term_width = term_size::dimensions()
        .map(|(w, _)| w)
        .unwrap_or(80);

    // ── Title ─────────────────────────────
    println!(
        "{}",
        Style::new().bold().fg(Cyan).paint(&note.meta.title)
    );

    // ── Tags ──────────────────────────────
    if !note.meta.tags.is_empty() {
        let tags = note
            .meta
            .tags
            .iter()
            .map(|t| Cyan.paint(t).to_string())
            .collect::<Vec<_>>()
            .join(" • ");

        println!("{}", tags);
    }

    println!();

    // ── Dates ─────────────────────────────
    let now = Local::now();
    let created = HumanTime::from(note.meta.created - now);
    let updated = HumanTime::from(note.meta.updated - now);

    println!(
        "{} {}",
        Fixed(8).paint("Created:"),
        Fixed(7).paint(created.to_string())
    );
    println!(
        "{} {}",
        Fixed(8).paint("Updated:"),
        Fixed(7).paint(updated.to_string())
    );

    println!("\n{}", "─".repeat(term_width.min(60)));

    // ── Body ──────────────────────────────
    let wrapped = fill(&note.body, term_width - 2);
    println!("{}", wrapped);
}

pub fn view_note(query: String, path: &str) -> anyhow::Result<()> {
    let parsed = read_note(query, path)?;
    render_note(parsed);
    Ok(())
}
