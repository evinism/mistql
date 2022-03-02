use mistql::{MistQLParser, Rule};

const _GRAMMAR: &str = include_str!("../../src/mistql.pest");

#[test]
fn parses_prefix_operators() {
    parses_to! {
        parser: MistQLParser,
        input: "!true",
        rule: Rule::query,
        tokens: [
            prefixed_value(0,5, [
                not_op(0,1),
                bool(1,5)
            ])
        ]
    }
}

#[test]
fn parses_prefix_operators_with_space() {
    parses_to! {
        parser: MistQLParser,
        input: "! true",
        rule: Rule::query,
        tokens: [
            prefixed_value(0,6, [
                not_op(0,1),
                bool(2,6)
            ])
        ]
    }
}

#[test]
fn parses_doubled_prefix_operators() {
    parses_to! {
        parser: MistQLParser,
        input: "!!true",
        rule: Rule::query,
        tokens: [
            prefixed_value(0,6, [
                not_op(0,1),
                prefixed_value(1,6, [
                    not_op(1,2),
                    bool(2,6)
                ])
            ])
        ]
    }
}

#[test]
fn parses_prefix_operator_on_expression() {
    parses_to! {
        parser: MistQLParser,
        input: "!!(regex \"hi\")",
        rule: Rule::query,
        tokens: [
            prefixed_value(0,14, [
                not_op(0,1),
                prefixed_value(1,14, [
                    not_op(1,2),
                    function(3,13, [
                        ident(3,8),
                        string(9,13, [
                            inner(10,12)
                        ])
                    ])
                ])
            ])
        ]
    }
}

#[test]
fn parses_prefix_operator_on_ident() {
    // it's weird that this is legal
    parses_to! {
        parser: MistQLParser,
        input: "!!float",
        rule: Rule::query,
        tokens: [
            prefixed_value(0,7, [
                not_op(0,1),
                prefixed_value(1,7, [
                    not_op(1,2),
                    ident(2,7)
                ])
            ])
        ]
    }
}
