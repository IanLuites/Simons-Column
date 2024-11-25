//! Benchmark to run

use std::num::NonZeroUsize;

/// Arguments to pass the benchmark.
#[derive(Debug, PartialEq, Eq)]
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
    /// Create arguments from a generic iterator.

    #[must_use]
    fn from_iter<T: std::fmt::Debug + std::string::ToString>(
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
#[derive(Debug, PartialEq, Eq)]
pub struct Iterations {
    /// Warmup iterations.
    ///
    /// `0` warmup means skip warmup.
    warmup: usize,

    /// Benchmark iterations.
    benchmark: NonZeroUsize,
}

/// Benchmark to run.
#[derive(Debug)]
pub struct Benchmark {
    /// Benchmark id
    id: String,

    /// Benchmark label.
    label: String,

    /// Variants of the benchmark with different arguments.
    variants: Vec<Arguments>,

    /// Benchmark iterations.
    iterations: Iterations,
}

impl Benchmark {
    /// Build a new benchmark.
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
            variants: Vec::new(),
            benchmark_warmup: None,
            benchmark_iterations: None,
        }
    }

    /// Benchmark id
    #[must_use]
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Benchmark label
    #[must_use]
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Variants
    #[must_use]
    pub fn variants(&self) -> &[Arguments] {
        &self.variants
    }

    /// Iterations
    #[must_use]
    pub const fn iterations(&self) -> &Iterations {
        &self.iterations
    }
}

/// Benchmark to run.
#[derive(Debug)]
pub struct Builder {
    /// Benchmark id
    id: String,

    /// Benchmark label.
    label: Option<String>,

    /// Variants of the benchmark with different arguments.
    variants: Vec<Arguments>,

    /// Warmup iterations.
    benchmark_warmup: Option<usize>,

    /// Benchmark iterations.
    benchmark_iterations: Option<NonZeroUsize>,
}

impl Builder {
    /// Build the benchmark.
    #[must_use]
    pub fn build(self) -> Benchmark {
        let iterations = self
            .benchmark_iterations
            .unwrap_or_else(|| NonZeroUsize::new(100).expect("100 to be non zero"));

        Benchmark {
            label: self.label.unwrap_or_else(|| {
                let mut c = self.id.chars();
                let f = c.next().expect("id to be not empty");
                f.to_uppercase().collect::<String>() + c.as_str()
            }),
            id: self.id,
            variants: if self.variants.is_empty() {
                vec![Arguments::none()]
            } else {
                self.variants
            },
            iterations: Iterations {
                warmup: self
                    .benchmark_warmup
                    .unwrap_or_else(|| iterations.ilog10() as usize),
                benchmark: iterations,
            },
        }
    }

    /// Set label for benchmark.
    #[must_use]
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());

        assert!(
            self.label.as_ref().is_some_and(|v| !v.is_empty()),
            "Empty label is not allowed."
        );

        self
    }

    /// Add a variant with given arguments.
    #[must_use]
    pub fn with_variant(mut self, arguments: impl Into<Arguments>) -> Self {
        let arguments = arguments.into();

        if !(arguments.is_none() || self.variants.contains(&arguments)) {
            self.variants.push(arguments);
        }

        self
    }

    /// Set warmup iterations.
    ///
    /// Set to `0` to disable warmup.
    #[must_use]
    pub fn with_warmup(mut self, warmup: impl Into<usize>) -> Self {
        self.benchmark_warmup = Some(warmup.into());
        self
    }

    /// Set benchmark iterations.
    ///
    /// Can not be `0`.
    #[must_use]
    pub fn with_iterations(mut self, iterations: impl Into<usize>) -> Self {
        self.benchmark_iterations =
            Some(NonZeroUsize::new(iterations.into()).expect("more than 0 benchmark iterations"));

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_benchmark() {
        let benchmark = Benchmark::build("bui ld")
            .with_label("Build Test")
            .with_variant(["hello"])
            .with_warmup(23usize)
            .with_iterations(500usize)
            .build();

        assert_eq!(benchmark.id(), "build");
        assert_eq!(benchmark.label(), "Build Test");
        assert_eq!(benchmark.variants, [Arguments(vec!["hello".into()])]);

        assert_eq!(
            benchmark.iterations(),
            &Iterations {
                warmup: 23,
                benchmark: NonZeroUsize::new(500).unwrap()
            }
        );
    }

    #[test]
    fn build_uses_id_for_label_if_no_label_set() {
        let benchmark = Benchmark::build("label").build();

        assert_eq!(benchmark.id(), "label");
        assert_eq!(benchmark.label(), "Label");
    }

    #[test]
    fn build_uses_warmup_of_log10_benchmarks_by_default() {
        let benchmark = Benchmark::build("warmup")
            .with_iterations(100_usize)
            .build();

        assert_eq!(benchmark.iterations().warmup, 2);

        let benchmark = Benchmark::build("warmup")
            .with_iterations(100_000_usize)
            .build();

        assert_eq!(benchmark.iterations().warmup, 5);
    }

    // TODO: Should potentially not panic.
    // TODO: Evaluate best approach: panic, warn, or error-tuple.
    #[test]
    #[should_panic(expected = "Id needs to be a none empty alphanumeric + '_' string.")]
    fn panics_if_empty_id() {
        let _ = Benchmark::build("");
    }

    // TODO: Should potentially not panic.
    // TODO: Evaluate best approach: panic, warn, or error-tuple.
    #[test]
    #[should_panic(expected = "Id needs to be a none empty alphanumeric + '_' string.")]
    fn panics_if_non_alphanumeric_empty_id() {
        let _ = Benchmark::build("!!");
    }

    // TODO: Should potentially not panic.
    // TODO: Evaluate best approach: panic, warn, or error-tuple.
    #[test]
    #[should_panic(expected = "Empty label is not allowed.")]
    fn panics_if_empty_label() {
        let _ = Benchmark::build("empty_label").with_label("");
    }

    // TODO: Should potentially not panic.
    // TODO: Evaluate best approach: panic, warn, or error-tuple.
    #[test]
    #[should_panic(expected = "Empty argument: \"\"")]
    fn panics_if_empty_argument() {
        let _ = Benchmark::build("Panic empty arg").with_variant([""]);
    }

    // TODO: Should potentially not panic.
    // TODO: Evaluate best approach: panic, warn, or error-tuple.
    #[test]
    #[should_panic(expected = "more than 0 benchmark iterations")]
    fn panics_if_0_benchmark_iterations() {
        let _ = Benchmark::build("Panic 0 iterations").with_iterations(0_usize);
    }

    #[test]
    fn builder_filters_duplicate_arguments() {
        let benchmark = Benchmark::build("duplicates")
            .with_variant(["some a"])
            .with_variant(["some b"])
            .with_variant(["some a"])
            .with_variant(["some c"])
            .build();

        assert_eq!(benchmark.variants().len(), 3);
        assert!(benchmark.variants().contains(&["some a"].into()));
        assert!(benchmark.variants().contains(&["some b"].into()));
        assert!(benchmark.variants().contains(&["some c"].into()));
    }
}
