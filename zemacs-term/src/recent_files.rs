//! Persistent most-recently-used (MRU) file list backing the startify start screen.
//!
//! Stored as newline-separated absolute paths at `<config-dir>/recent_files`,
//! newest first. Files are recorded on `DocumentDidOpen` (see
//! `handlers::recent_files`) and read when the start screen is shown on launch
//! with no file argument (see `ui::startify`). This is the equivalent of vim's
//! `v:oldfiles` / startify's MRU section, which Helix has no native store for.

use std::path::{Path, PathBuf};

const FILE_NAME: &str = "recent_files";
const MAX_ENTRIES: usize = 50;

fn store_path() -> PathBuf {
    zemacs_loader::config_dir().join(FILE_NAME)
}

/// Load the recent-files list, newest first. A missing or unreadable store yields
/// an empty list. Entries that no longer exist on disk are filtered out so the
/// start screen never offers a dead path.
pub fn load() -> Vec<PathBuf> {
    let Ok(contents) = std::fs::read_to_string(store_path()) else {
        return Vec::new();
    };
    contents
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(PathBuf::from)
        .filter(|p| p.is_file())
        .take(MAX_ENTRIES)
        .collect()
}

/// Record `path` as the most-recently-used file: canonicalizes it, moves it to
/// the front (deduping), caps the list, and writes it back. Non-files (scratch
/// buffers, directories) are ignored.
pub fn record(path: &Path) {
    if !path.is_file() {
        return;
    }
    let path = std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());

    let mut entries = load();
    entries.retain(|p| p != &path);
    entries.insert(0, path);
    entries.truncate(MAX_ENTRIES);

    let body = entries
        .iter()
        .map(|p| p.to_string_lossy().into_owned())
        .collect::<Vec<_>>()
        .join("\n");

    let store = store_path();
    if let Some(parent) = store.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let _ = std::fs::write(store, body);
}
