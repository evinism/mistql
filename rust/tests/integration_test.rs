use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Deserialize)]
struct TestSuite {
    data: Vec<TestSuiteMember>
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum TestSuiteMember {
    TestGroup { describe: String, cases: Vec<TestSuiteMember> },
    Test { it: String, assertions: Vec<Assertion> }
}

#[derive(Debug, Deserialize)]
struct Assertion {
    query: String,
    data: serde_json::Value,
    #[serde(default = "default_expected")]
    expected: serde_json::Value,
    #[serde(default = "default_throws")]
    throws: bool
}

fn default_expected() -> serde_json::Value {
    serde_json::Value::Null
}

fn default_throws() -> bool {
    false
}

#[test]
fn shared_tests() {
    let file = File::open("../shared/testdata.json").unwrap();
    let reader = BufReader::new(file);

    let tests: TestSuite = serde_json::from_reader(reader).unwrap();

    

    dbg!(tests);
}