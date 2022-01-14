use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Deserialize)]
struct TestSuite {
    data: Vec<TestSuiteMember>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
enum TestSuiteMember {
    TestGroup {
        describe: String,
        cases: Vec<TestSuiteMember>,
    },
    Test {
        it: String,
        assertions: Vec<Assertion>,
    },
}

#[derive(Clone, Debug, Deserialize)]
struct Assertion {
    query: String,
    data: serde_json::Value,
    #[serde(default = "default_expected")]
    expected: serde_json::Value,
    #[serde(default = "default_throws")]
    throws: bool,
}

#[derive(Clone, Debug)]
struct TestResult {
    name: String,
    passed: bool,
}

fn default_expected() -> serde_json::Value {
    serde_json::Value::Null
}

fn default_throws() -> bool {
    false
}

fn run_assertion(assertion: Assertion) -> bool {
    let result = mistql::query_value(assertion.query, assertion.data);
    match result {
        Ok(res) => res == assertion.expected,
        Err(_) => false,
    }
}

fn run_test(name: String, assertions: Vec<Assertion>) -> Vec<TestResult> {
    assertions
        .iter()
        .map(|assertion| TestResult {
            name: format! {"{} - {}", name, assertion.query},
            passed: run_assertion(assertion.clone()),
        })
        .collect()
}

fn run_test_group(name: String, members: Vec<TestSuiteMember>) -> Vec<TestResult> {
    members
        .iter()
        .map(|case| match case {
            TestSuiteMember::Test { it, assertions } => {
                run_test(format!("{} - {}", name, it), assertions.to_vec())
            }
            TestSuiteMember::TestGroup { describe, cases } => {
                run_test_group(format!("{} - {}", name, describe), cases.to_vec())
            }
        })
        .flatten()
        .collect()
}

#[test]
fn run_shared_tests() {
    let file = File::open("../shared/testdata.json").unwrap();
    let reader = BufReader::new(file);

    let tests: TestSuite = serde_json::from_reader(reader).unwrap();
    let results = run_test_group("".to_string(), tests.data);

    let failures: Vec<TestResult> = results
        .iter()
        .filter(|res| !res.passed)
        .cloned()
        .collect::<Vec<TestResult>>();
    assert!(
        failures.is_empty(),
        "{} integration tests failed:\n{}",
        failures.len(),
        failures
            .iter()
            .map(|failure| failure.name.clone())
            .collect::<Vec<String>>()
            .join("\n")
    )
}
