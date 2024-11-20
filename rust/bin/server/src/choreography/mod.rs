//! Choreography of lights

mod storage;
pub use storage::Storage;

/// Light choreography
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct Choreography {
    /// Choreography name.
    name: String,

    /// Format
    format: Format,

    /// Data
    data: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub enum Format {
    /// Python3 light choreography.
    Python,
}

impl Choreography {
    /// Create a new choreography.
    #[must_use]
    #[cfg(test)]
    pub fn new(name: impl AsRef<str>, format: Format, data: impl AsRef<str>) -> Self {
        Self {
            name: name.as_ref().into(),
            format,
            data: data.as_ref().into(),
        }
    }

    /// Choreography name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Choreography format.
    #[must_use]
    pub const fn format(&self) -> Format {
        self.format
    }

    /// Compiles choreography to Python.
    pub fn compile(&self) -> String {
        match self.format {
            Format::Python => self.data.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Create test choreography.
    fn test_choreography(format: Format, data: impl AsRef<str>) -> Choreography {
        Choreography {
            name: "Test".into(),
            format,
            data: data.as_ref().into(),
        }
    }

    #[test]
    fn compile_from_python() {
        let choreography = test_choreography(Format::Python, "print('Hello world!')");

        assert_eq!(choreography.compile(), "print('Hello world!')");
    }
}
