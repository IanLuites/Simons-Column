//! Collected results from all benchmarks

use std::collections::{hash_map::Entry, HashMap};

use crate::{benchmark::Case, implementation::Implementation};

use super::output::Timings;

/// Collected results from all benchmarks
#[derive(Debug)]
pub struct Results {
    /// Device identifier.
    device: String,

    /// Benchmark definitions.
    benchmarks: HashMap<String, String>,

    /// Collected timings for each benchmark case.
    timings: HashMap<String, CaseTimings>,
}

/// Timings for a single benchmark case indexed by implementation.
#[derive(Debug, Default)]
struct CaseTimings(HashMap<String, Timings>);

impl Default for Results {
    fn default() -> Self {
        /// Raspberry Pi host a model name under
        /// `/proc/device-tree/model` or `/sys/firmware/devicetree/base/model`
        const PI_DEVICE_PATH: &str = "/proc/device-tree/model";

        let device = if std::fs::exists(PI_DEVICE_PATH).is_ok_and(|e| e) {
            let mut bytes: Vec<u8> = std::fs::read(PI_DEVICE_PATH).expect("Pi identifier.");
            if let Some(delimiter_0) = bytes.iter().position(|&b| b == 0) {
                bytes.truncate(delimiter_0);
            }

            String::from_utf8(bytes).expect("Valid UTF-8")
        } else {
            "Unknown".into()
        };

        Self {
            device,
            benchmarks: HashMap::default(),
            timings: HashMap::default(),
        }
    }
}

impl Results {
    /// Record benchmark result.
    pub fn record(&mut self, case: &Case, implementation: &Implementation, timings: Timings) {
        if let Entry::Vacant(entry) = self.benchmarks.entry(case.id()) {
            let iterations = case.iterations();
            entry.insert(format!(r#"{{"label":{label:?},"arguments":[{arguments}],"warmup":{warmup},"benchmark":{benchmark}}}"#, label = case.label(), arguments = case.arguments().into_iter().map(|arg|format!("{arg:?}")).collect::<Vec<String>>().join(","), warmup = iterations.warmup(), benchmark = iterations.benchmark()));
        }

        match self.timings.entry(case.id()) {
            Entry::Vacant(entry) => {
                let mut case_timings = CaseTimings::default();
                case_timings.0.insert(implementation.id().into(), timings);
                entry.insert(case_timings);
            }
            Entry::Occupied(mut entry) => {
                entry
                    .get_mut()
                    .0
                    .insert(implementation.id().into(), timings);
            }
        }
    }

    /// Turn results into json.
    pub fn to_json(&self) -> String {
        let mut json = format!(r#"{{"device":{:#?},"benchmarks":{{"#, self.device,);
        let mut first_case = true;

        for (case, details) in &self.benchmarks {
            if first_case {
                first_case = false;
            } else {
                json.push(',');
            }

            json.push('"');
            json.push_str(case);
            json.push('"');
            json.push(':');
            json.push_str(details);
        }

        json.push('}');
        first_case = true;
        json.push(',');
        json.push_str(r#""timings":{"#);

        for (case, case_timings) in &self.timings {
            if first_case {
                first_case = false;
            } else {
                json.push(',');
            }

            json.push('"');
            json.push_str(case);
            json.push('"');
            json.push(':');

            json.push('{');
            let mut first_implementation = true;

            for (implementation, timings) in &case_timings.0 {
                if first_implementation {
                    first_implementation = false;
                } else {
                    json.push(',');
                }

                json.push('"');
                json.push_str(implementation);
                json.push('"');
                json.push(':');

                json.push('{');
                json.push_str(&format!(
                    r#""warmup":{},"benchmark":{},"execution":{}"#,
                    timings.warmup, timings.benchmark, timings.execution
                ));
                json.push('}');
            }

            json.push('}');
        }
        json.push('}');
        json.push('}');
        json
    }
}
