//! Implementations of benchmarks

use std::path::PathBuf;

use crate::benchmark::Suite;

/// Implementation of benchmarks.
#[derive(Debug, PartialEq, Eq)]
pub struct Implementation {
    /// Implementation id
    id: String,

    /// Implementation label.
    label: String,

    /// Implementation directory.
    directory: PathBuf,
}

impl Implementation {
    /// Build a new implementation.
    #[must_use]
    pub fn build(id: impl AsRef<str>) -> Builder {
        let clean_id = id
            .as_ref()
            .chars()
            .filter(|c| *c == '_' || c.is_ascii_alphanumeric())
            .collect::<String>()
            .to_ascii_lowercase();

        assert!(
            !clean_id.is_empty(),
            "Id needs to be a none empty alphanumeric + '_' string."
        );

        Builder {
            id: clean_id,
            label: None,
            directory: None,
        }
    }

    /// Implementation id
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Implementation label
    #[must_use]
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Checks whether an implementation implements the given benchmark.
    #[must_use]
    pub fn implements(&self, benchmark: &Suite) -> bool {
        self.implementation_file(benchmark).is_some()
    }

    /// Find the implementation file for the benchmark.
    #[must_use]
    pub fn implementation_file(&self, benchmark: &Suite) -> Option<PathBuf> {
        let mut file = self.directory.join(benchmark.id());

        if file.exists() {
            return Some(file);
        }

        file.set_extension("py");

        if file.exists() {
            Some(file)
        } else {
            None
        }
    }
}

/// Implementation builder.
#[derive(Debug)]
pub struct Builder {
    /// Implementation id
    id: String,

    /// Implementation label.
    label: Option<String>,

    /// Implementation directory.
    directory: Option<PathBuf>,
}

impl Builder {
    /// Build the implementation.
    #[must_use]
    pub fn build(self) -> Implementation {
        Implementation {
            label: self.label.unwrap_or_else(|| {
                let mut c = self.id.chars();
                let f = c.next().expect("id to be not empty");
                f.to_uppercase().collect::<String>() + c.as_str()
            }),
            directory: self.directory.unwrap_or_else(|| {
                let current = std::env::current_dir().expect("current working directory");
                current.join(&self.id)
            }),
            id: self.id,
        }
    }

    /// Set label for benchmark.
    #[must_use]
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());

        assert!(
            self.label.as_ref().is_some_and(|v| !v.is_empty()),
            "Empty label is not allowed."
        );

        self
    }

    /// Set implementation directory.
    #[must_use]
    pub fn directory(mut self, directory: impl Into<PathBuf>) -> Self {
        self.directory = Some(directory.into());
        self
    }
}
