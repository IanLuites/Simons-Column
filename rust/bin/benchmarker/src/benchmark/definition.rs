//! Benchmark definition

use std::num::NonZeroUsize;

use crate::util::clean_id;

/// Arguments to pass the benchmark.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Arguments(Vec<String>);

impl Arguments {
    /// No arguments.
    #[must_use]
    pub const fn none() -> Self {
        Self(vec![])
    }

    /// Check for no arguments.
    #[must_use]
    pub fn is_none(&self) -> bool {
        self.0.is_empty()
    }

    /// Add an argument.
    pub fn add(&mut self, argument: impl Into<String>) {
        self.0.push(argument.into());
    }

    /// Append arguments.
    pub fn append(&mut self, arguments: impl Into<Self>) {
        self.0.append(&mut (arguments.into().0));
    }

    /// Checks whether an argument is contained in the arguments.
    #[must_use]
    pub fn contains(&self, argument: impl Into<String>) -> bool {
        self.0.contains(&argument.into())
    }

    /// Create arguments from a generic iterator.
    #[must_use]
    pub(crate) fn from_iter<T: std::fmt::Debug + std::string::ToString>(
        iter: impl Iterator<Item = T>,
    ) -> Self {
        let mut arguments = Vec::new();

        for item in iter {
            let arg = item.to_string();
            assert!(!arg.is_empty(), "Empty argument: {item:#?}");

            arguments.push(arg);
        }

        Self(arguments)
    }
}

// Implement IntoIterator to allow `Arguments` to be passed to <Command>.args()
impl std::iter::IntoIterator for &Arguments {
    type Item = String;
    type IntoIter = <Vec<String> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.clone().into_iter()
    }
}

/// Create implementation to convert a given type to args based on iter.
macro_rules! args_from_iter {
    ($($t:ty),*) => {
        $(
            impl From<$t> for Arguments {
                fn from(value: $t) -> Self {
                    Self::from_iter(value.into_iter())
                }
            }
        )*
    };
}

args_from_iter!(
    &[&str],
    Vec<&str>,
    Vec<String>,
    // Bit of convenience below
    [&str; 1],
    [&str; 2],
    [&str; 3]
);

/// Iterations configuration for benchmark.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Iterations {
    /// Warmup iterations.
    ///
    /// `0` warmup means skip warmup.
    warmup: Option<usize>,

    /// Benchmark iterations.
    benchmark: Option<NonZeroUsize>,
}

impl Iterations {
    #[doc(hidden)]
    #[must_use]
    pub const fn new(warmup: Option<usize>, benchmark: Option<NonZeroUsize>) -> Self {
        Self { warmup, benchmark }
    }

    #[doc(hidden)]
    #[must_use]
    pub const fn has_warmup(&self) -> bool {
        self.warmup.is_some()
    }

    #[doc(hidden)]
    #[must_use]
    pub const fn has_benchmark(&self) -> bool {
        self.benchmark.is_some()
    }

    /// Warmup iterations.
    #[must_use]
    pub fn warmup(&self) -> usize {
        self.warmup
            .map_or_else(|| self.benchmark().ilog10() as usize * 2, |v| v)
    }

    /// Benchmark iterations.
    #[must_use]
    pub fn benchmark(&self) -> NonZeroUsize {
        self.benchmark
            .unwrap_or(NonZeroUsize::new(100_usize).unwrap())
    }
}

impl std::ops::BitOrAssign for Iterations {
    fn bitor_assign(&mut self, rhs: Self) {
        self.warmup = self.warmup.or(rhs.warmup);
        self.benchmark = self.benchmark.or(rhs.benchmark);
    }
}

impl std::ops::BitOr for Iterations {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            warmup: self.warmup.or(rhs.warmup),
            benchmark: self.benchmark.or(rhs.benchmark),
        }
    }
}

// Implement IntoIterator to allow `Arguments` to be passed to <Command>.args()
// This one is definitely not as clean
// TODO: Refactor and reconsider
impl std::iter::IntoIterator for &Iterations {
    type Item = String;
    type IntoIter = <Vec<String> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        vec![self.warmup().to_string(), self.benchmark().to_string()].into_iter()
    }
}

/// Benchmark definition.
#[derive(Debug, PartialEq, Eq)]
pub struct Definition {
    /// Benchmark id
    pub(super) id: String,

    /// Benchmark label.
    pub(super) label: String,

    /// Benchmark arguments
    pub(super) arguments: Arguments,

    /// Benchmark iterations.
    pub(super) iterations: Iterations,
}

/// Builder for `Definition`.
#[derive(Debug, Default)]
pub(super) struct Builder {
    /// Definition Id.
    id: String,

    /// Definition label.
    label: Option<String>,

    /// Arguments.
    arguments: Arguments,

    /// Iterations.
    iterations: Iterations,
}

impl Definition {
    /// Build a new benchmark definition.
    #[must_use]
    pub(super) fn build(id: impl AsRef<str>) -> Builder {
        let clean_id =
            clean_id(id).expect("Id needs to be a none empty alphanumeric + '_' string.");

        Builder {
            id: clean_id,
            ..Default::default()
        }
    }
}

impl Builder {
    /// Set the label.
    #[must_use]
    pub fn label(mut self, label: impl Into<String>) -> Self {
        let label = label.into();
        assert!(!label.is_empty(), "Empty label is not allowed.");

        self.label = Some(label);
        self
    }

    /// Add an argument.
    #[must_use]
    pub fn arg(mut self, argument: impl Into<String>) -> Self {
        self.arguments.add(argument);
        self
    }

    /// Add arguments.
    #[must_use]
    pub fn args(mut self, arguments: impl Into<Arguments>) -> Self {
        self.arguments.append(arguments);
        self
    }

    /// Set the warmup iterations.
    #[must_use]
    pub const fn warmup(mut self, warmup: usize) -> Self {
        self.iterations.warmup = Some(warmup);
        self
    }

    /// Set the benchmark iterations.
    #[must_use]
    pub const fn iterations(mut self, iterations: usize) -> Self {
        self.iterations.benchmark = NonZeroUsize::new(iterations);
        self
    }

    /// Build the `Definition`.
    ///
    /// # Panics
    ///
    /// Panics if any of the required fields are missing.
    #[must_use]
    pub fn build(self) -> Definition {
        Definition {
            label: self.label.unwrap_or_else(|| {
                let mut c = self.id.chars();
                let f = c.next().expect("id to be not empty");
                f.to_uppercase().collect::<String>() + c.as_str()
            }),
            id: self.id,
            arguments: self.arguments,
            iterations: self.iterations,
        }
    }
}
