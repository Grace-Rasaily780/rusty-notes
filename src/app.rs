use crate::cli::{Cli, Commands};
use crate::note;
use crate::fs::paths;
use std::path::PathBuf;

pub fn run(cli: Cli) -> anyhow::Result<()> {
    match cli.command {
        Commands::New { title, tag, .. } => {
            note::create::create(title, tag, &cli.dir, cli.editor.clone())?;
        }

        Commands::List { .. } => {
                let path = paths::notes_dir(&cli.dir);
                let notes_paths = note::finder::list_notes(&path.to_str().expect("Path contains invalid UTF-8"))
                    .into_iter()
                    .map(PathBuf::from)
                    .collect();
                
                if let Some(note_path) = note::finder::interactive_list(notes_paths)? {
                    // Call your open_note() or read_note() here
                    let note = crate::note::read::read_note_file(note_path.as_path())?;
                     crate::note::read::render_note(note);
                    
                }          
              }

        Commands::Open { query, editor, .. } => {
            crate::note::open::open_note(query, &cli.dir, editor)?;
        }

        Commands::Read { query, .. } => {
            let path = paths::notes_dir(&cli.dir);
            let _ = crate::note::read::view_note(query, &path.to_str().expect("Path contains invalid UTF-8"));
        }

        _ => println!("Command not implemented yet"),
    }

    Ok(())
}
