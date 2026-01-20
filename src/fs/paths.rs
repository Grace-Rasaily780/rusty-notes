use std::path::PathBuf;

pub fn notes_dir(base: &str) -> PathBuf {
    shellexpand::tilde(base).into_owned().into()
}

pub fn inbox_dir(base: &str) -> PathBuf {
    notes_dir(base).join("inbox")
}
