//! Single benchmark case

use super::{Arguments, Definition, Iterations};

/// Single benchmark case.
#[derive(Debug, PartialEq, Eq)]
pub struct Case<'a> {
    /// Single benchmark.
    case: &'a Definition,

    /// The benchmark suite.
    namespace: Option<&'a Definition>,
}

impl Case<'_> {
    /// Case Id.
    #[must_use]
    pub fn id(&self) -> String {
        self.namespace.map_or_else(
            || self.case.id.clone(),
            |namespace| format!("{}.{}", namespace.id, self.case.id),
        )
    }

    /// Case label.
    #[must_use]
    pub fn label(&self) -> String {
        self.namespace.map_or_else(
            || self.case.label.clone(),
            |namespace| format!("{}: {}", namespace.label, self.case.label),
        )
    }

    /// Benchmark arguments.
    #[must_use]
    pub fn arguments(&self) -> Arguments {
        self.namespace
            .map_or(self.case.arguments.clone(), |namespace| {
                let mut args = namespace.arguments.clone();
                args.append(self.case.arguments.clone());
                args
            })
    }

    /// Benchmark iterations.
    #[must_use]
    pub fn iterations(&self) -> Iterations {
        self.namespace.map_or(self.case.iterations, |namespace| {
            self.case.iterations | namespace.iterations
        })
    }
}

#[doc(hidden)]
#[derive(Debug)]
enum CaseIteratorInner<'a> {
    Single(std::iter::Once<&'a Definition>),
    Matrix {
        namespace: &'a Definition,
        iter: std::slice::Iter<'a, Definition>,
    },
}

/// Case iterator to iterate through cases in a suite.
#[derive(Debug)]
#[allow(clippy::module_name_repetitions)]
pub struct CaseIterator<'a>(CaseIteratorInner<'a>);

impl<'a> CaseIterator<'a> {
    /// Case iterator over a suite with a single case. (no matrix)
    #[must_use]
    pub(super) fn single(definition: &'a Definition) -> Self {
        Self(CaseIteratorInner::Single(std::iter::once(definition)))
    }

    /// Case iterator over a suite with a matrix of cases.
    #[must_use]
    pub(super) fn matrix(namespace: &'a Definition, definitions: &'a [Definition]) -> Self {
        Self(CaseIteratorInner::Matrix {
            namespace,
            iter: definitions.iter(),
        })
    }

    #[cfg(test)]
    /// Check whether a case iterator contains a specific case.
    #[must_use]
    pub fn contains(&mut self, case: &Case) -> bool {
        self.any(|f| &f == case)
    }
}

impl<'a> Iterator for CaseIterator<'a> {
    type Item = Case<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.0 {
            CaseIteratorInner::Single(definition) => Some(Case {
                case: definition.next()?,
                namespace: None,
            }),
            CaseIteratorInner::Matrix { namespace, iter } => Some(Case {
                namespace: Some(namespace),
                case: iter.next()?,
            }),
        }
    }
}
