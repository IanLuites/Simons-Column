//! Simple file base choreography storage.

use std::{
    fs::{File, OpenOptions},
    path::{Path, PathBuf},
};

use super::{Choreography, Format};

/// Simple file base choreography storage.
#[derive(Debug)]
pub struct Storage {
    /// Storage directory.
    directory: PathBuf,
}

impl Storage {
    /// Open storage.
    #[must_use]
    pub fn open(directory: impl AsRef<Path>) -> Self {
        let directory = directory.as_ref().to_path_buf();

        std::fs::create_dir_all(&directory).expect("to create storage directory");

        Self { directory }
    }

    /// List all stored choreographies.
    #[must_use]
    pub fn list(&self) -> Vec<Choreography> {
        let mut files = vec![];
        let dir = std::fs::read_dir(&self.directory).expect("reading storage directory");

        for path in dir.flatten() {
            if let Ok(file) = File::open(path.path()) {
                if let Ok(choreography) = serde_json::from_reader(file) {
                    files.push(choreography);
                }
            }
        }

        files
    }

    /// Read a choreography from storage.
    #[must_use]
    pub fn read(&self, name: impl AsRef<str>) -> Option<Choreography> {
        if let Ok(file) = File::open(self.name_to_path(name.as_ref())) {
            if let Ok(data) = serde_json::from_reader(file) {
                return Some(data);
            }
        }

        None
    }

    /// Write choreography to storage.
    pub fn write(&self, choreography: &Choreography) {
        let file = self.file(choreography, File::options().create(true).write(true));
        serde_json::to_writer_pretty(file, choreography).expect("write JSON choreography file");
    }

    /// Update choreography to storage.
    ///
    /// See `rename` for changing choreography name.
    pub fn update(
        &self,
        mut choreography: Choreography,
        format: Format,
        data: impl AsRef<str>,
    ) -> Choreography {
        let file = self.file(&choreography, File::options().truncate(true).write(true));
        choreography.format = format;
        choreography.data = data.as_ref().into();

        serde_json::to_writer_pretty(file, &choreography).expect("write JSON choreography file");
        choreography
    }

    /// Rename a choreography.
    pub fn rename(
        &self,
        mut choreography: Choreography,
        new_name: impl AsRef<str>,
    ) -> Choreography {
        let name: String = new_name.as_ref().into();

        if Self::name_to_id(&name) != Self::name_to_id(choreography.name()) {
            let old = self.path(&choreography);
            choreography.name = name;

            self.write(&choreography);
            std::fs::remove_file(old).expect("to remove rename choreography");
        }

        choreography
    }

    /// Choreography path
    #[must_use]
    fn path(&self, choreography: &Choreography) -> std::path::PathBuf {
        self.name_to_path(choreography.name())
    }

    /// Choreography path
    #[must_use]
    fn name_to_path(&self, name: &str) -> std::path::PathBuf {
        let id = Self::name_to_id(name);
        self.directory.join(format!("{id}.json"))
    }

    /// Sanitize a choreography name to Id.
    #[must_use]
    fn name_to_id(name: &str) -> String {
        name.to_ascii_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect()
    }

    /// File
    #[must_use]
    fn file(&self, choreography: &Choreography, options: &OpenOptions) -> File {
        options
            .open(self.path(choreography))
            .expect("choreography file access")
    }
}

#[cfg(test)]
mod tests {
    use crate::choreography::Format;

    use super::*;

    const TEST_DIRECTORY: &str = "./test";

    fn cleanup() {
        if let Err(err) = std::fs::remove_dir_all(TEST_DIRECTORY) {
            assert!(
                err.kind() == std::io::ErrorKind::NotFound,
                "Failed to clean test directory."
            );
        }
    }

    fn test_dir() -> std::path::PathBuf {
        cleanup();
        TEST_DIRECTORY.into()
    }

    #[test]
    fn storage_use() {
        let storage = Storage::open(test_dir());
        assert_eq!(storage.list(), vec![]);

        let choreography = Choreography::new("Test", Format::Python, "print('Hello world')");
        storage.write(&choreography);

        assert_eq!(storage.list(), vec![choreography.clone()]);
        assert_eq!(storage.read("test"), Some(choreography.clone()));
        assert_eq!(storage.read("TeSt"), Some(choreography.clone()));
        assert_eq!(storage.read("other"), None);

        let choreography = storage.rename(choreography, "new");
        assert_eq!(storage.read("test"), None);
        assert_eq!(storage.read("new"), Some(choreography.clone()));

        let choreography = storage.update(choreography, Format::Python, "print('Goodbye!')");
        assert_eq!(choreography.data, "print('Goodbye!')");
        assert_eq!(storage.read("new"), Some(choreography));

        cleanup();
    }
}
