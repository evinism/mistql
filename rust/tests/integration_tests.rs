#[macro_use]
extern crate pest;

use mistql::{error::ErrorKind, Rule};
use pest::Parser;
use serde::Deserialize;
use std::fmt;
use std::fs::File;
use std::io::BufReader;

#[derive(Clone, Copy)]
enum TestMode {
    Parse,
    Assert,
}

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
    msg: String,
}

impl fmt::Display for TestResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {}", self.name, self.msg)
    }
}

fn default_expected() -> serde_json::Value {
    serde_json::Value::Null
}

fn default_throws() -> bool {
    false
}

fn parse_query(name: String, assertion: &Assertion) -> TestResult {
    let result = mistql::MistQLParser::parse(Rule::query, &assertion.query);
    match result {
        Ok(_) => TestResult {
            name: name,
            passed: true,
            msg: "parsed".to_string(),
        },
        Err(_) if assertion.throws => TestResult {
            name: name,
            passed: true,
            msg: "expected fail".to_string(),
        },
        Err(err) => TestResult {
            name: name,
            passed: false,
            msg: format!("parser error {:?}", err),
        },
    }
}

fn run_assertion(name: String, assertion: &Assertion) -> TestResult {
    let result = mistql::query_value(assertion.query.clone(), assertion.data.clone());
    match result {
        Ok(res) if assertion.throws => TestResult {
            name: name,
            passed: false,
            msg: format!("expected error, got {}", res),
        },
        Ok(res) if res != assertion.expected => TestResult {
            name: name,
            passed: false,
            msg: format!("expected {} got {}", assertion.expected, res),
        },
        Ok(_res) => TestResult {
            name: name,
            passed: true,
            msg: "passed".to_string(),
        },
        Err(err) if assertion.throws => match err.kind {
            ErrorKind::Unimplemented(msg) => TestResult {
                name: name,
                passed: false,
                msg: msg,
            },
            _ => TestResult {
                name: name,
                passed: true,
                msg: "passed".to_string(),
            },
        },
        Err(err) => TestResult {
            name: name,
            passed: false,
            msg: format!("expected {}, got error {}", assertion.expected, err),
        },
    }
}

fn run_test(name: String, assertions: Vec<Assertion>, mode: TestMode) -> Vec<TestResult> {
    assertions
        .iter()
        .map(|assertion| match mode {
            TestMode::Parse => parse_query(format! {"{} - {}", name, assertion.query}, assertion),
            TestMode::Assert => {
                run_assertion(format! {"{} - {}", name, assertion.query}, assertion)
            }
        })
        .collect()
}

fn run_test_group(name: String, members: Vec<TestSuiteMember>, mode: TestMode) -> Vec<TestResult> {
    members
        .iter()
        .map(|case| match case {
            TestSuiteMember::Test { it, assertions } => {
                run_test(format!("{} - {}", name, it), assertions.to_vec(), mode)
            }
            TestSuiteMember::TestGroup { describe, cases } => {
                run_test_group(format!("{} - {}", name, describe), cases.to_vec(), mode)
            }
        })
        .flatten()
        .collect()
}

#[test]
fn parse_shared_tests() {
    let file = File::open("shared/testdata.json").unwrap();
    let reader = BufReader::new(file);

    let tests: TestSuite = serde_json::from_reader(reader).unwrap();
    let results = run_test_group("".to_string(), tests.data, TestMode::Parse);

    let failures: Vec<TestResult> = results
        .iter()
        .filter(|res| !res.passed)
        .cloned()
        .collect::<Vec<TestResult>>();
    assert!(
        failures.is_empty(),
        "{}\n\n{}/{} integration tests failed",
        failures
            .iter()
            .map(|r| r.to_string())
            .collect::<Vec<String>>()
            .join("\n"),
        failures.len(),
        results.len(),
    )
}

#[test]
fn run_shared_tests() {
    let file = File::open("../shared/testdata.json").unwrap();
    let reader = BufReader::new(file);

    let tests: TestSuite = serde_json::from_reader(reader).unwrap();
    let results = run_test_group("".to_string(), tests.data, TestMode::Assert);

    let failures: Vec<TestResult> = results
        .iter()
        .filter(|res| !res.passed)
        .cloned()
        .collect::<Vec<TestResult>>();
    assert!(
        failures.is_empty(),
        "{}\n\n{}/{} integration tests failed",
        failures
            .iter()
            .map(|r| r.to_string())
            .collect::<Vec<String>>()
            .join("\n"),
        failures.len(),
        results.len(),
    )
}
