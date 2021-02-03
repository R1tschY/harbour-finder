use std::fs;
use std::path::PathBuf;
use std::rc::Rc;

use log::info;
use xdg_desktop_entry::{discover_applications, DesktopEntry, StandardKey};

pub struct Entry {
    pub name: String,
    pub icon: Option<String>,
    pub exec: String,
    pub file_name: String,
}

pub struct DesktopEntryIndex {
    entries: Vec<Rc<Entry>>,
}

impl DesktopEntryIndex {
    pub fn new() -> Self {
        Self { entries: vec![] }
    }

    pub fn index(&mut self) {
        info!("Indexing ...");
        self.entries = discover_applications()
            .into_iter()
            .filter_map(|entry| self.parse_desktop_entry(&entry))
            .collect();
        info!("Found {} entries.", self.entries.len(),);
    }

    fn parse_desktop_entry(&self, path: &PathBuf) -> Option<Rc<Entry>> {
        info!("Reading {:?} ...", &path);
        let content = fs::read_to_string(&path).ok()?;
        let desktop_entry = DesktopEntry::parse_string(&content).ok()?;
        Some(Rc::new(Entry {
            name: desktop_entry
                .localized_get(StandardKey::Name.key_name(), &None)?
                .to_string(),
            icon: desktop_entry
                .localized_get(StandardKey::Icon.key_name(), &None)
                .map(|icon| icon.to_string()),
            exec: desktop_entry
                .localized_get(StandardKey::Exec.key_name(), &None)?
                .to_string(),
            file_name: path
                .file_name()
                .unwrap_or_default()
                .to_str()
                .map(|path| path.to_string())
                .unwrap_or_default(),
        }))
    }

    pub fn get_all(&self) -> Vec<Rc<Entry>> {
        self.entries.iter().cloned().collect()
    }

    pub fn search_for(&self, query: &str) -> Vec<Rc<Entry>> {
        self.entries
            .iter()
            .filter(|entry| entry.name.starts_with(query))
            .cloned()
            .collect()
    }
}
