use crate::cli::{Cli, Commands};
use crate::note;
use crate::fs::paths;

pub fn run(cli: Cli) -> anyhow::Result<()> {
    match cli.command {
        Commands::New { title, tag, .. } => {
            note::create::create(title, tag, &cli.dir, cli.editor.clone())?;
        }

        Commands::List { .. } => {
                let path = paths::notes_dir(&cli.dir);
                let notes = note::finder::list_notes(&path.to_str().expect("Path contains invalid UTF-8"));
                for note in notes {
                    println!("{}", note);
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
