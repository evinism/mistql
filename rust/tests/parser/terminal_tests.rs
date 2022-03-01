use mistql::{MistQLParser, Rule};

const _GRAMMAR: &str = include_str!("../../src/mistql.pest");

#[test]
fn parses_at() {
    parses_to! {
        parser: MistQLParser,
        input: "@",
        rule: Rule::query,
        tokens: [
            at(0,1)
        ]
    }
}

#[test]
fn parses_null() {
    parses_to! {
        parser: MistQLParser,
        input: "null",
        rule: Rule::query,
        tokens: [
            null(0,4)
        ]
    }
}

#[test]
fn parses_true() {
    parses_to! {
        parser: MistQLParser,
        input: "true",
        rule: Rule::query,
        tokens: [
            bool(0,4)
        ]
    }
}

#[test]
fn parses_false() {
    parses_to! {
        parser: MistQLParser,
        input: "false",
        rule: Rule::query,
        tokens: [
            bool(0,5)
        ]
    }
}

#[test]
fn parses_ident() {
    parses_to! {
        parser: MistQLParser,
        input: "float",
        rule: Rule::query,
        tokens: [
            ident(0,5)
        ]
    }
}

#[test]
fn ident_doesnt_begin_with_integer() {
    fails_with! {
        parser: MistQLParser,
        input: "12float",
        rule: Rule::ident,
        positives: vec![Rule::ident],
        negatives: vec![],
        pos: 0
    }
}

#[test]
fn parses_positive_integer() {
    let query = "100000";
    parses_to! {
        parser: MistQLParser,
        input: query.clone(),
        rule: Rule::query,
        tokens: [
            number(0,6)
        ]
    }
}

#[test]
fn parses_negative_integer() {
    let query = "-100000";
    parses_to! {
        parser: MistQLParser,
        input: query,
        rule: Rule::query,
        tokens: [
            number(0,7)
        ]
    }
}

#[test]
fn parses_zero() {
    let query = "0";
    parses_to! {
        parser: MistQLParser,
        input: query,
        rule: Rule::query,
        tokens: [
            number(0,1)
        ]
    }
}

#[test]
fn parses_float() {
    let query = "30.5";
    parses_to! {
        parser: MistQLParser,
        input: query,
        rule: Rule::query,
        tokens: [
            number(0,4)
        ]
    }
}

#[test]
fn parses_float_with_leading_zero() {
    let query = "0.9";
    parses_to! {
        parser: MistQLParser,
        input: query,
        rule: Rule::query,
        tokens: [
            number(0,3)
        ]
    }
}

#[test]
fn parses_negative_float() {
    let query = "-30.5";
    parses_to! {
        parser: MistQLParser,
        input: query,
        rule: Rule::query,
        tokens: [
            number(0,5)
        ]
    }
}

#[test]
fn parses_float_with_exponent() {
    parses_to! {
        parser: MistQLParser,
        input: "4.9E50",
        rule: Rule::query,
        tokens: [
            number(0,6)
        ]
    }
}

#[test]
fn parses_negative_float_with_exponent() {
    parses_to! {
        parser: MistQLParser,
        input: "-30.5e-2",
        rule: Rule::query,
        tokens: [
            number(0,8)
        ]
    }
}

#[test]
fn parses_a_string() {
    parses_to! {
        parser: MistQLParser,
        input: "\"hello\"",
        rule: Rule::query,
        tokens: [
            string(0,7, [
                inner(1,6)
            ])
        ]
    }
}

#[test]
fn parse_escaped_quotes() {
    parses_to! {
        parser: MistQLParser,
        input: "\"\"",
        rule: Rule::query,
        tokens: [
            string(0,2, [
                inner(1,1)
            ])
        ]
    }
}

#[test]
fn parse_escaped_escapes() {
    parses_to! {
        parser: MistQLParser,
        input: "\"\\\"\"",
        rule: Rule::query,
        tokens: [
            string(0,4, [
                inner(1,3)
            ])
        ]
    }
}

#[test]
fn parse_unicodes() {
    parses_to! {
        parser: MistQLParser,
        input: "\"\\u0022\\\\\\\"\"",
        rule: Rule::query,
        tokens: [
            string(0,12, [
                inner(1,11)
            ])
        ]
    }
}

#[test]
fn parse_all_the_escapes() {
    parses_to! {
        parser: MistQLParser,
        input: "\"\\u0022\\\\\\\"\\b\\r\\n\"",
        rule: Rule::query,
        tokens: [
            string(0,18, [
                inner(1,17)
            ])
        ]
    }
}

#[test]
fn parse_double_escapes() {
    parses_to! {
        parser: MistQLParser,
        input: "\"\\\\s\"",
        rule: Rule::query,
        tokens: [
            string(0,5, [
                inner(1,4)
            ])
        ]
    }
}
