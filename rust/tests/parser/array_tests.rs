use mistql::parse::{MistQLParser, Rule};

const _GRAMMAR: &str = include_str!("../../src/mistql.pest");

#[test]
fn parses_empty_array() {
    parses_to! {
        parser: MistQLParser,
        input: "[]",
        rule: Rule::query,
        tokens: [
            array(0,2)
        ]
    }
}

#[test]
fn parses_array_of_integers() {
    parses_to! {
        parser: MistQLParser,
        input: "[1,2,3]",
        rule: Rule::query,
        tokens: [
            array(0,7, [
                number(1,2),
                number(3,4),
                number(5,6)
            ])
        ]
    }
}

#[test]
fn parses_array_of_strings() {
    parses_to! {
        parser: MistQLParser,
        input: "[\"a\",\"b\",\"c\"]",
        rule: Rule::query,
        tokens: [
            array(0,13, [
                string(1,4, [inner(2,3)]),
                string(5,8, [inner(6,7)]),
                string(9,12, [inner(10,11)])
            ])
        ]
    }
}

#[test]
fn parses_array_of_booleans() {
    parses_to! {
        parser: MistQLParser,
        input: "[true, false, true]",
        rule: Rule::query,
        tokens: [
            array(0,19, [
                bool(1,5),
                bool(7,12),
                bool(14,18)
            ])
        ]
    }
}

#[test]
fn parses_array_of_nulls() {
    parses_to! {
        parser: MistQLParser,
        input: "[null, null]",
        rule: Rule::query,
        tokens: [
            array(0,12, [
                null(1,5),
                null(7,11),
            ])
        ]
    }
}

#[test]
fn parses_array_with_mixed_types() {
    parses_to! {
        parser: MistQLParser,
        input: "[null, true, 3, \"d\"]",
        rule: Rule::query,
        tokens: [
            array(0,20, [
                null(1,5),
                bool(7,11),
                number(13,14),
                string(16,19, [inner(17,18)])
            ])
        ]
    }
}

#[test]
fn parses_array_of_arrays() {
    parses_to! {
        parser: MistQLParser,
        input: "[[1,2],3]",
        rule: Rule::query,
        tokens: [
            array(0,9, [
                array(1,6, [
                    number(2,3),
                    number(4,5),
                ]),
                number(7,8)
            ])
        ]
    }
}

#[test]
fn fails_to_parse_unterminated_array() {
    fails_with! {
        parser: MistQLParser,
        input: "[",
        rule: Rule::query,
        positives: vec![
            Rule::function, Rule::indexed_value, Rule::prefix_op, Rule::compound_reference,
            Rule::object, Rule::array, Rule::ident, Rule::string, Rule::number, Rule::bool,
            Rule::null, Rule::at, Rule::dollar
        ],
        negatives: vec![],
        pos: 1
    }
}

#[test]
fn fails_to_parse_unterminated_array_with_contents() {
    fails_with! {
        parser: MistQLParser,
        input: "[1,2,3",
        rule: Rule::query,
        positives: vec![Rule::pipe, Rule::infix_op],
        negatives: vec![],
        pos: 6
    }
}
