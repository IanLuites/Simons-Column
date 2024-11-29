//! Run a benchmark on an implementation.

use std::{path::PathBuf, process::Command};

use crate::{
    benchmark::{Case, Suite},
    implementation::Implementation,
};

use super::output::{Output, Timings};

/// Check whether a file exists with or without .py extension.
#[inline]
#[must_use]
fn exist_or_py_exist(path: &mut PathBuf) -> bool {
    if path.exists() {
        return true;
    }

    path.set_extension("py");

    path.exists()
}

/// File implementing a specific benchmark case.
fn implementation_file(implementation: &Implementation, case: &Case) -> Option<PathBuf> {
    let mut base = implementation.directory().join(case.suite_id());
    let mut case = base.join(case.id());

    if exist_or_py_exist(&mut base) {
        Some(base)
    } else if exist_or_py_exist(&mut case) {
        Some(case)
    } else {
        None
    }
}

/// Iterate over all cases an implementation implements.
pub fn implements_cases<'a>(
    implementation: &'a Implementation,
    suite: &'a Suite,
) -> impl Iterator<Item = Case<'a>> + 'a {
    suite
        .cases()
        .filter(|case| implementation_file(implementation, case).is_some())
}

/// Run a benchmark case on an implementation.
#[must_use]
pub fn run(implementation: &Implementation, case: &Case) -> Output {
    let Some(file) = implementation_file(implementation, case) else {
        return Output::implementation_not_found();
    };

    let mut command = if file.extension().is_some_and(|ext| ext == "py") {
        let mut command = Command::new("python3");
        command.arg(&file);
        command
    } else {
        Command::new(&file)
    };

    command.args(&case.iterations());
    command.args(&case.arguments());

    let start = std::time::Instant::now();
    match command.output() {
        Ok(output) => {
            let duration = start.elapsed().as_nanos();
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();

            if output.status.success() {
                if let Some(timings) = timings(duration, &stdout) {
                    Output::success(stdout + &stderr, timings)
                } else {
                    Output::failure(stdout + &stderr + "\nMISSING TIMINGS\n")
                }
            } else {
                Output::failure(stdout + &stderr)
            }
        }
        Err(error) => Output::failure(format!("{error:#?}")),
    }
}

/// Parse timings from output.
#[must_use]
fn timings(execution: u128, stdout: &str) -> Option<Timings> {
    let mut warmup = None;
    let mut benchmark = None;

    for line in stdout.lines().rev() {
        if line.trim().is_empty() {
            continue;
        }

        if let Some(bench) = line.strip_prefix("benchmark: ") {
            benchmark = Some(bench.parse().ok()?);
        } else if let Some(bench) = line.strip_prefix("warmup: ") {
            warmup = Some(bench.parse().ok()?);
        } else {
            return None;
        }

        if benchmark.is_some() && warmup.is_some() {
            break;
        }
    }

    Some(Timings {
        execution,
        warmup: warmup.unwrap(),
        benchmark: benchmark.unwrap(),
    })
}
