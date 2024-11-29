//! Benchmarks to run

mod case;
mod definition;
mod suite;

pub use case::{Case, CaseIterator};
pub use definition::{Arguments, Definition, Iterations};
pub use suite::Suite;

use definition::Builder as DefinitionBuilder;

/// Benchmark suite builder.
#[derive(Debug, Default)]
pub struct Builder {
    /// Benchmark namespace for matrix or single case.
    namespace: DefinitionBuilder,

    /// Matrix of benchmark cases.
    cases: Vec<Definition>,
}

/// Benchmark case builder.
#[derive(Debug)]
pub struct CaseBuilder {
    /// Definition of case.
    inner: DefinitionBuilder,
}

impl Suite {
    /// Build a new benchmark suite.
    #[must_use]
    pub fn build(id: impl AsRef<str>) -> Builder {
        Builder {
            namespace: Definition::build(id),
            ..Default::default()
        }
    }
}

impl Builder {
    /// Build a benchmark suite.
    #[must_use]
    pub fn build(self) -> Suite {
        if self.cases.is_empty() {
            Suite::single(self.namespace.build())
        } else {
            Suite::matrix(self.namespace.build(), self.cases)
        }
    }

    /// Add a case to the benchmark suite.
    #[must_use]
    pub fn case(
        mut self,
        id: impl AsRef<str>,
        case: impl FnOnce(CaseBuilder) -> CaseBuilder,
    ) -> Self {
        let case = case(CaseBuilder {
            inner: Definition::build(id),
        })
        .inner
        .build();

        if !self.cases.iter().any(|c| c.id == case.id) {
            self.cases.push(case);
        }

        self
    }
}

/// Expose definition builder methods on case or suite builder.
macro_rules! expose_builder {
    ($($builder:ty => $field:ident),*) => {
        $(
            impl $builder {
                /// Set the label.
                #[must_use]
                pub fn label(mut self, label: impl Into<String>) -> Self {
                    self.$field = self.$field.label(label);
                    self
                }

                /// Add an argument.
                #[must_use]
                pub fn arg(mut self, argument: impl Into<String>) -> Self {
                    self.$field = self.$field.arg(argument);
                    self
                }

                /// Add arguments.
                #[must_use]
                pub fn args(mut self, arguments: impl Into<Arguments>) -> Self {
                    self.$field = self.$field.args(arguments);
                    self
                }

                /// Set the warmup iterations.
                #[must_use]
                pub fn warmup(mut self, warmup: usize) -> Self {
                    self.$field = self.$field.warmup(warmup);
                    self
                }

                /// Set the benchmark iterations.
                #[must_use]
                pub fn iterations(mut self, iterations: usize) -> Self {
                    self.$field = self.$field.iterations(iterations);
                    self
                }
            }
        )*
    };
}

expose_builder!(Builder => namespace, CaseBuilder => inner);

#[cfg(test)]
mod tests {
    use std::num::NonZeroUsize;

    use super::*;

    #[test]
    fn build_benchmark() {
        let suite = Suite::build("bui ld")
            .label("Build Test")
            .arg("hello")
            .warmup(23usize)
            .iterations(500usize)
            .build();

        for case in &suite {
            assert_eq!(case.id(), "build");
            assert_eq!(case.label(), "Build Test");
            assert_eq!(case.arguments(), ["hello"].into());

            assert_eq!(
                case.iterations(),
                Iterations::new(Some(23), NonZeroUsize::new(500))
            );
        }
    }

    #[test]
    fn build_uses_id_for_label_if_no_label_set() {
        let benchmark = Suite::build("label").build();

        assert_eq!(benchmark.id(), "label");
        assert_eq!(benchmark.label(), "Label");
    }

    #[test]
    fn build_uses_warmup_base_on_benchmarks_by_default() {
        let benchmark = Suite::build("warmup").iterations(100_usize).build();
        let case = benchmark.into_iter().next().expect("first");
        assert_eq!(case.iterations().warmup(), 4);

        let benchmark = Suite::build("warmup").iterations(100_000_usize).build();
        let case = benchmark.into_iter().next().expect("first");
        assert_eq!(case.iterations().warmup(), 10);
    }

    // TODO: Should potentially not panic.
    // TODO: Evaluate best approach: panic, warn, or error-tuple.
    #[test]
    #[should_panic(expected = "Id needs to be a none empty alphanumeric + '_' string.")]
    fn panics_if_empty_id() {
        let _ = Suite::build("");
    }

    // TODO: Should potentially not panic.
    // TODO: Evaluate best approach: panic, warn, or error-tuple.
    #[test]
    #[should_panic(expected = "Id needs to be a none empty alphanumeric + '_' string.")]
    fn panics_if_non_alphanumeric_empty_id() {
        let _ = Suite::build("!!");
    }

    // TODO: Should potentially not panic.
    // TODO: Evaluate best approach: panic, warn, or error-tuple.
    #[test]
    #[should_panic(expected = "Empty label is not allowed.")]
    fn panics_if_empty_label() {
        let _ = Suite::build("empty_label").label("");
    }

    // TODO: Should potentially not panic.
    // TODO: Evaluate best approach: panic, warn, or error-tuple.
    #[test]
    #[should_panic(expected = "Id needs to be a none empty alphanumeric + '_' string.")]
    fn panics_if_empty_argument() {
        let _ = Suite::build("Panic empty arg").case("", |case| case);
    }

    #[test]
    fn builder_filters_duplicate_arguments() {
        let benchmark = Suite::build("duplicates")
            .case("a", |case| case.arg("some a"))
            .case("b", |case| case.arg("some b"))
            .case("c", |case| case.arg("some c"))
            .case("a", |case| case.arg("some a"))
            .build();

        assert_eq!(benchmark.cases().count(), 3);

        assert!(benchmark.cases().any(|c| c.arguments().contains("some a")));
        assert!(benchmark.cases().any(|c| c.arguments().contains("some b")));
        assert!(benchmark.cases().any(|c| c.arguments().contains("some c")));
    }
}
