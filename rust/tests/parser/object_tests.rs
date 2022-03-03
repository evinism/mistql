use mistql::{MistQLParser, Rule};

const _GRAMMAR: &str = include_str!("../../src/mistql.pest");

#[test]
fn parses_empty_object() {
    parses_to! {
        parser: MistQLParser,
        input: "{}",
        rule: Rule::query,
        tokens: [
            object(0,2)
        ]
    }
}

#[test]
fn parses_object_with_string_keys() {
    parses_to! {
        parser: MistQLParser,
        input: "{\"a\": 1, \"b\": 2, \"c\": 3}",
        rule: Rule::query,
        tokens: [
            object(0,24, [
                keyval(1,7, [
                    string(1,4, [inner(2,3)]),
                    number(6,7)
                ]),
                keyval(9,15, [
                    string(9,12, [inner(10,11)]),
                    number(14,15)
                ]),
                keyval(17,23, [
                    string(17,20, [inner(18,19)]),
                    number(22,23)
                ])
            ])
        ]
    }
}

#[test]
fn parses_object_with_mixed_type_values() {
    parses_to! {
        parser: MistQLParser,
        input: "{\"a\": 1, \"b\": false, \"c\": \"three\"}",
        rule: Rule::query,
        tokens: [
            object(0,34, [
                keyval(1,7, [
                    string(1,4, [inner(2,3)]),
                    number(6,7)
                ]),
                keyval(9,19, [
                    string(9,12, [inner(10,11)]),
                    bool(14,19)
                ]),
                keyval(21,33, [
                    string(21,24, [inner(22,23)]),
                    string(26,33, [inner(27,32)])
                ])
            ])
        ]
    }
}

#[test]
fn parses_object_with_unqouted_keys() {
    parses_to! {
        parser: MistQLParser,
        input: "{a: 1, b: 2, c: 3}",
        rule: Rule::query,
        tokens: [
            object(0,18, [
                keyval(1,5, [
                    ident(1,2),
                    number(4,5)
                ]),
                keyval(7,11, [
                    ident(7,8),
                    number(10,11)
                ]),
                keyval(13,17, [
                    ident(13,14),
                    number(16,17)
                ])
            ])
        ]
    }
}

#[test]
fn fails_to_parse_unterminated_object() {
    fails_with! {
        parser: MistQLParser,
        input: "{",
        rule: Rule::query,
        positives: vec![Rule::keyval],
        negatives: vec![],
        pos: 1
    }
}

#[test]
fn fails_to_parse_unterminated_object_with_contents() {
    fails_with! {
        parser: MistQLParser,
        input: "{a: 1, b: 2",
        rule: Rule::query,
        positives: vec![
            Rule::plus_op, Rule::minus_op, Rule::mult_op, Rule::div_op, Rule::mod_op,
            Rule::eq_op, Rule::ne_op, Rule::gte_op, Rule::gt_op, Rule::lte_op, Rule::lt_op,
            Rule::and_op, Rule::or_op, Rule::match_op
        ],
        negatives: vec![],
        pos: 11
    }
}
