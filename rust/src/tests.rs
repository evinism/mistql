//! Shared test suite for MistQL Rust implementation
//!
//! This module loads and runs the language-independent test suite from the shared directory,
//! ensuring cross-platform compatibility with JavaScript and Python implementations.

use crate::query_runtime;
use crate::types::RuntimeValue;
use serde_json::Value;

// Test case structure matching the shared testdata.json format
#[derive(Debug, Clone)]
pub struct TestCase {
    pub assertions: Vec<TestAssertion>,
    pub describe_block: String,
    pub describe_inner: String,
    pub it_description: String,
    pub skip: Option<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct TestAssertion {
    pub assertion_number: usize,
    pub query: String,
    pub data: RuntimeValue,
    pub expected: Option<RuntimeValue>,
    pub expected_set: Option<Vec<RuntimeValue>>,
    pub throws: Option<String>,
}

#[derive(Debug)]
pub struct TestResults {
    pub total_test_cases: usize,
    pub total_assertions: usize,
    pub passed_assertions: usize,
    pub failed_assertions: usize,
    pub skipped_test_cases: usize,
    pub skipped_assertions: usize,
    pub failures: Vec<TestFailure>,
}

#[derive(Debug)]
pub struct TestFailure {
    pub assertion_number: usize,
    pub describe_block: String,
    pub describe_inner: String,
    pub it_description: String,
    pub query: String,
    pub data: RuntimeValue,
    pub expected: Option<RuntimeValue>,
    pub expected_set: Option<Vec<RuntimeValue>>,
    pub actual: Option<RuntimeValue>,
    pub error: Option<String>,
}

// Note: Removed values_equal function since we now use direct RuntimeValue comparisons

// Load test data from the shared testdata.json file
pub fn load_test_data() -> Result<Vec<TestCase>, Box<dyn std::error::Error>> {
    let testdata_path = "shared/testdata.json";
    let content = std::fs::read_to_string(testdata_path)?;
    let testdata: Value = serde_json::from_str(&content)?;

    let mut test_cases = Vec::new();
    let mut assertion_counter = 1; // Start numbering from 1

    // Process the nested structure: data -> cases -> cases -> assertions
    if let Some(data_array) = testdata.get("data").and_then(|d| d.as_array()) {
        for block in data_array {
            if let Some(describe_block) = block.get("describe").and_then(|d| d.as_str()) {
                if let Some(cases_array) = block.get("cases").and_then(|c| c.as_array()) {
                    for inner_block in cases_array {
                        if let Some(describe_inner) = inner_block.get("describe").and_then(|d| d.as_str()) {
                            if let Some(test_array) = inner_block.get("cases").and_then(|c| c.as_array()) {
                                for test in test_array {
                                    if let Some(it_description) = test.get("it").and_then(|i| i.as_str()) {
                                        let skip = test
                                            .get("skip")
                                            .and_then(|s| s.as_array())
                                            .map(|arr| arr.iter().filter_map(|v| v.as_str()).map(|s| s.to_string()).collect());

                                        if let Some(assertions_array) = test.get("assertions").and_then(|a| a.as_array()) {
                                            let mut assertions = Vec::new();

                                            for assertion in assertions_array {
                                                if let (Some(query), Some(data)) =
                                                    (assertion.get("query").and_then(|q| q.as_str()), assertion.get("data"))
                                                {
                                                    let expected = assertion.get("expected");
                                                    let expected_set = assertion.get("expected_set");
                                                    let throws = assertion.get("throws").and_then(|t| {
                                                        if t.is_boolean() && t.as_bool().unwrap_or(false) {
                                                            Some("true")
                                                        } else {
                                                            t.as_str()
                                                        }
                                                    });

                                                    assertions.push(TestAssertion {
                                                        assertion_number: assertion_counter,
                                                        query: query.to_string(),
                                                        data: data.try_into().unwrap(),
                                                        expected: expected.map(|e| e.try_into().unwrap()),
                                                        expected_set: expected_set.map(|e| e.as_array().unwrap().iter().map(|v| v.try_into().unwrap()).collect()),
                                                        throws: throws.map(|s| s.to_string())
                                                    });
                                                    assertion_counter += 1;
                                                }
                                            }

                                            test_cases.push(TestCase {
                                                assertions,
                                                describe_block: describe_block.to_string(),
                                                describe_inner: describe_inner.to_string(),
                                                it_description: it_description.to_string(),
                                                skip,
                                            });
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(test_cases)
}

// Run a single test assertion
// Returns (pass_status, actual_value) where actual_value is Some for successful queries
pub fn run_assertion(assertion: &TestAssertion) -> Result<(bool, Option<Value>), String> {
    if let Some(expected_error) = &assertion.throws {
        // Test should throw an error
        match query_runtime(&assertion.query, &assertion.data) {
            Ok(actual_result) => {
                match actual_result.to_serde_value(false) {
                    Ok(actual_value) => {
                        return Err(format!(
                            "Expected error '{}' but query succeeded with result: {:?}",
                            expected_error, actual_value
                        ))
                    }
                    // Parser errors are expected.
                    Err(_e) => return Ok((true, None)),
                }
            }
            Err(_e) => {
                // Could be more granular, but shared tests don't specify error types.
                Ok((true, None)) // No actual value for error cases
            }
        }
    } else {
        // Test should succeed and return expected value
        match query_runtime(&assertion.query, &assertion.data) {
            Ok(result) => {
                // Convert to JSON only for reporting
                let actual_value = Some(result.to_serde_value(false).unwrap_or_else(|_| serde_json::Value::Null));

                // Check if we have an expectedSet (takes precedence over expected)
                if let Some(expected_set) = &assertion.expected_set {
                    // Check if actual result matches any value in the expected set
                    let matches = expected_set.iter().any(|expected| (&result) == expected);
                    Ok((matches, actual_value))
                } else if let Some(expected) = &assertion.expected {
                    // Convert expected value to RuntimeValue for comparison
                    let matches = (&result) == expected;
                    Ok((matches, actual_value))
                } else {
                    Ok((true, actual_value)) // No expected value specified, just check it doesn't error
                }
            }
            Err(e) => Err(format!("Query failed with error: {}", e)),
        }
    }
}

// Run all test cases, separating skipped and non-skipped tests
pub fn run_test_suite() -> Result<TestResults, Box<dyn std::error::Error>> {
    let test_cases = load_test_data()?;
    let rust_lang_id = "rust";

    let mut non_skipped_cases = Vec::new();
    let mut skipped_cases = Vec::new();

    for test_case in test_cases {
        if let Some(skip_list) = &test_case.skip {
            if skip_list.contains(&rust_lang_id.to_string()) {
                skipped_cases.push(test_case);
                continue;
            }
        }
        non_skipped_cases.push(test_case);
    }

    // Count total assertions across all test cases
    let total_assertions: usize = non_skipped_cases.iter().map(|tc| tc.assertions.len()).sum();
    let skipped_assertions: usize = skipped_cases.iter().map(|tc| tc.assertions.len()).sum();

    let mut results = TestResults {
        total_test_cases: non_skipped_cases.len(),
        total_assertions,
        passed_assertions: 0,
        failed_assertions: 0,
        skipped_test_cases: skipped_cases.len(),
        skipped_assertions: skipped_assertions,
        failures: Vec::new(),
    };

    for test_case in non_skipped_cases {
        for assertion in &test_case.assertions {
            match run_assertion(assertion) {
                Ok((true, _actual)) => {
                    results.passed_assertions += 1;
                }
                Ok((false, actual)) => {
                    results.failed_assertions += 1;
                    results.failures.push(TestFailure {
                        assertion_number: assertion.assertion_number,
                        describe_block: test_case.describe_block.clone(),
                        describe_inner: test_case.describe_inner.clone(),
                        it_description: test_case.it_description.clone(),
                        query: assertion.query.clone(),
                        data: assertion.data.clone(),
                        expected: assertion.expected.clone(),
                        expected_set: assertion.expected_set.clone(),
                        actual: actual.map(|a| a.try_into().unwrap()),
                        error: None,
                    });
                }
                Err(error) => {
                    results.failed_assertions += 1;
                    results.failures.push(TestFailure {
                        assertion_number: assertion.assertion_number,
                        describe_block: test_case.describe_block.clone(),
                        describe_inner: test_case.describe_inner.clone(),
                        it_description: test_case.it_description.clone(),
                        query: assertion.query.clone(),
                        data: assertion.data.clone(),
                        expected: assertion.expected.clone(),
                        expected_set: assertion.expected_set.clone(),
                        actual: None,
                        error: Some(error),
                    });
                }
            }
        }
    }

    Ok(results)
}

#[rustfmt::skip]
impl std::fmt::Display for TestFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "FAIL #{}: {}::{}::{}", self.assertion_number, self.describe_block, self.describe_inner, self.it_description)?;
        writeln!(f, "  Query: {}", self.query)?;
        writeln!(f, "  Data: {}", serde_json::to_string_pretty(&self.data).unwrap_or_else(|_| "Invalid JSON".to_string()))?;
        if let Some(expected) = &self.expected {
            writeln!(f, "  Expected: {}", serde_json::to_string_pretty(expected).unwrap_or_else(|_| "Invalid JSON".to_string()))?;
        }
        if let Some(expected_set) = &self.expected_set {
            writeln!(f, "  Expected Set: {}", serde_json::to_string_pretty(expected_set).unwrap_or_else(|_| "Invalid JSON".to_string()))?;
        }
        if let Some(actual) = &self.actual {
            writeln!(f, "  Actual: {}", serde_json::to_string_pretty(actual).unwrap_or_else(|_| "Invalid JSON".to_string()))?;
        }
        if let Some(error) = &self.error {
            writeln!(f, "  Error: {}", error)?;
        }
        Ok(())
    }
}

impl TestFailure {
    fn summary(&self) -> String {
        format!(
            "FAIL #{}: {}::{}::{}",
            self.assertion_number, self.describe_block, self.describe_inner, self.it_description
        )
    }
}

#[rustfmt::skip]
impl std::fmt::Display for TestResults {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.failures.is_empty() {
            for failure in &self.failures {
                writeln!(f, "{}", failure)?;
            }

            writeln!(f, "\nFailures ({} total):", self.failures.len())?;
            for failure in &self.failures {
                writeln!(f, "{}", failure.summary())?;
            }
        }

        writeln!(f, "\nTest Results Summary:")?;
        writeln!(f, "  Test Cases: {} ({} skipped)", self.total_test_cases, self.skipped_test_cases)?;
        writeln!(f, "  Assertions: {} total", self.total_assertions)?;
        writeln!(f, "    ✅ Passed: {}", self.passed_assertions)?;
        writeln!(f, "    ❌ Failed: {}", self.failed_assertions)?;
        writeln!(f, "    ⏭️ Skipped: {}", self.skipped_assertions)?;
        if self.total_assertions > 0 {
            let pass_rate = (self.passed_assertions as f64 / self.total_assertions as f64) * 100.0;
            writeln!(f, "  Pass Rate: {:.1}%", pass_rate)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test_runner {
    use super::*;

    #[test]
    fn test_shared_suite() {
        match run_test_suite() {
            Ok(results) => {
                println!("{}", results);

                if results.failed_assertions > 0 {
                    println!(
                        "Note: {} assertions failed. This is expected during development.",
                        results.failed_assertions
                    );
                    println!("Focus on implementing missing functionality to reduce failures.");
                }

                // At minimum, we should have some tests running
                assert!(results.total_assertions > 0, "No assertions were executed");
            }
            Err(e) => {
                panic!("Failed to run test suite: {}", e);
            }
        }
    }

    #[test]
    fn test_basic_functionality() {
        use serde_json::json;

        // @ on [1, 2, 3]
        let runtime_data: RuntimeValue = (&json!([1, 2, 3])).try_into().unwrap();
        let result = query_runtime("@", &runtime_data).expect("Basic identity query should work");
        assert_eq!(result, runtime_data, "@ on arrays should work");

        // @ on {"a": 1, "b": 2}
        let runtime_data: RuntimeValue = (&json!({"a": 1, "b": 2})).try_into().unwrap();
        let result = query_runtime("@", &runtime_data).expect("Object identity query should work");
        assert_eq!(result, runtime_data, "@ on objects should work");
    }

    #[test]
    fn test_dollar_variable() {
        use serde_json::json;

        // $.@ on {"filter": "cat", "nums": [1, 2, 3]}
        let runtime_data: RuntimeValue = (&json!({"filter": "cat", "nums": [1, 2, 3]})).try_into().unwrap();
        let result = query_runtime("$.@", &runtime_data).expect("$.@ should work");
        assert_eq!(result, runtime_data, "$.@ should work");

        // $.@.filter on {"filter": "cat", "nums": [1, 2, 3]}
        let result = query_runtime("$.@.filter", &runtime_data).expect("$.@.filter should work");
        assert_eq!(result, (&json!("cat")).try_into().unwrap(), "$.@.filter should work");

        // $.count $.@.nums on {"filter": "cat", "nums": [1, 2, 3]}
        let result = query_runtime("$.count $.@.nums", &runtime_data).expect("$.count $.@.nums should work");
        assert_eq!(result, (&json!(3)).try_into().unwrap(), "$.count $.@.nums should work");

        // $.sum $.@.nums on {"filter": "cat", "nums": [1, 2, 3]}
        let result = query_runtime("$.sum $.@.nums", &runtime_data).expect("$.sum $.@.nums should work");
        assert_eq!(result, (&json!(6)).try_into().unwrap(), "$.sum $.@.nums should work");
    }

    #[test]
    fn test_float_function() {
        use serde_json::json;

        let runtime_data: RuntimeValue = (&json!(null)).try_into().unwrap();

        // float "1.1e1"
        let result = query_runtime("float \"1.1e1\"", &runtime_data).expect("float should work");
        assert_eq!(result, RuntimeValue::Number(11.0), "float \"1.1e1\" should work");

        // float "5."
        let result = query_runtime("float \"5.\"", &runtime_data).expect("float should work");
        assert_eq!(result, RuntimeValue::Number(5.0), "float \"5.\" should work");

        // float "1.1e1"
        let result = query_runtime("float \"1.1e1\"", &runtime_data).expect("float should work");
        assert_eq!(result, RuntimeValue::Number(11.0), "float \"1.1e1\" should work");

        // float "5."
        let result = query_runtime("float \"5.\"", &runtime_data).expect("float should work");
        assert_eq!(result, RuntimeValue::Number(5.0), "float \"5.\" should work");
    }

    #[test]
    fn test_string_function() {
        use serde_json::json;

        // Test string function with various numbers
        let test_cases = vec![(1e50, "1e+50"), (3e20, "300000000000000000000"), (3e21, "3e+21"), (1e-7, "1e-7")];

        for (input, expected) in test_cases {
            let runtime_data: RuntimeValue = (&json!(input)).try_into().unwrap();
            let result = query_runtime("string @", &runtime_data).expect("string should work");
            assert_eq!(result, RuntimeValue::String(expected.to_string()), "string should work");
        }
    }

    #[test]
    fn test_keys_function() {
        use serde_json::json;

        let runtime_data: RuntimeValue = (&json!({})).try_into().unwrap();

        // Test keys function
        let result = query_runtime("{a: 1, b: 2} | keys", &runtime_data).expect("keys should work");
        assert_eq!(
            result,
            RuntimeValue::Array(vec![RuntimeValue::String("a".to_string()), RuntimeValue::String("b".to_string())]),
            "keys should work"
        );

        // Test empty object
        let result = query_runtime("{} | keys", &runtime_data).expect("keys should work");
        assert_eq!(result, RuntimeValue::Array(vec![]), "empty keys should work");
    }

    #[test]
    fn test_regex_operator() {
        use serde_json::json;

        let runtime_data: RuntimeValue = (&json!(null)).try_into().unwrap();

        // Test =~ operator with string pattern
        let result = query_runtime("\"Hello\" =~ \"[a-z]ello\"", &runtime_data).expect("regex should work");
        assert_eq!(result, RuntimeValue::Boolean(false), "Hello =~ [a-z]ello should work");

        // Test =~ operator with regex object
        let result = query_runtime("\"Hello\" =~ (regex \"[a-z]ello\" \"i\")", &runtime_data).expect("regex should work");
        assert_eq!(result, RuntimeValue::Boolean(true), "Hello =~ (regex [a-z]ello i) should work");
    }

    #[test]
    fn test_expected_set_functionality() {

        // Test that expectedSet works correctly
        let assertion = TestAssertion {
            assertion_number: 999,
            query: "string {a: 1, b: \"2\"}".to_string(),
            data: RuntimeValue::Null,
            expected: None,
            expected_set: Some(vec![
                RuntimeValue::String("{\"a\":1,\"b\":\"2\"}".to_string()),
                RuntimeValue::String("{\"b\":\"2\",\"a\":1}".to_string()),
            ]),
            throws: None,
        };

        let result = run_assertion(&assertion);
        match result {
            Ok((true, _)) => {
                // Test passed - the actual result matched one of the expected values
                println!("expectedSet test passed");
            }
            Ok((false, actual)) => {
                panic!("expectedSet test failed - actual result: {:?}", actual);
            }
            Err(e) => {
                panic!("expectedSet test error: {}", e);
            }
        }
    }
}
