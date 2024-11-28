//! Benchmark suite of a single case or a matrix

use super::{Case, CaseIterator, Definition};

/// Benchmark to run.
#[derive(Debug, PartialEq, Eq)]
pub enum Suite {
    /// Single benchmark case.
    Single(Definition),

    /// Matrix of benchmark cases.
    Matrix {
        /// Benchmark suite namespace (shared) settings.
        namespace: Definition,

        /// Benchmark cases.
        definitions: Vec<Definition>,
    },
}

impl Suite {
    /// Create a benchmark suite with a single case.
    #[must_use]
    pub(super) const fn single(definition: Definition) -> Self {
        Self::Single(definition)
    }

    /// Create a benchmark suite with a matrix of cases.
    #[must_use]
    pub(super) const fn matrix(namespace: Definition, definitions: Vec<Definition>) -> Self {
        Self::Matrix {
            namespace,
            definitions,
        }
    }

    /// Benchmark suite cases.
    #[must_use]
    pub fn cases(&self) -> <&Self as IntoIterator>::IntoIter {
        self.into_iter()
    }

    /// Benchmark suite Id.
    ///
    /// Note: this can be different from a specific case Id.
    #[must_use]
    pub fn id(&self) -> &str {
        match self {
            Self::Single(definition) => &definition.id,
            Self::Matrix { namespace, .. } => &namespace.id,
        }
    }

    /// Benchmark suite full label.
    #[must_use]
    pub fn label(&self) -> String {
        match self {
            Self::Single(definition) => definition.label.clone(),
            Self::Matrix {
                namespace,
                definitions,
            } => format!(
                "{} ({})",
                namespace.label,
                definitions
                    .iter()
                    .map(|d| d.label.as_str())
                    .collect::<Vec<&str>>()
                    .join(", ")
            ),
        }
    }

    /// Benchmark suite label
    #[must_use]
    pub fn short_label(&self) -> &str {
        match self {
            Self::Single(definition) => &definition.label,
            Self::Matrix { namespace, .. } => &namespace.label,
        }
    }
}

impl<'a> IntoIterator for &'a Suite {
    type Item = Case<'a>;
    type IntoIter = CaseIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Suite::Single(definition) => CaseIterator::single(definition),
            Suite::Matrix {
                namespace,
                definitions,
            } => CaseIterator::matrix(namespace, definitions),
        }
    }
}
