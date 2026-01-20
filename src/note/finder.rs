use walkdir::WalkDir;

pub fn list_notes(base: &str) -> Vec<String> {
  WalkDir::new(base)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map(|s| s == "note").unwrap_or(false))
        .map(|e| e.path().to_string_lossy().into_owned())
        .collect()
}
