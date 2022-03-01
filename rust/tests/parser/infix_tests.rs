use mistql::{MistQLParser, Rule};

const _GRAMMAR: &str = include_str!("../../src/mistql.pest");

#[test]
fn parses_infix_operators() {
    parses_to! {
        parser: MistQLParser,
        input: "1 + 3",
        rule: Rule::query,
        tokens: [
            number(0,1),
            infix_op(2,3),
            number(4,5)
        ]
    }
}

#[test]
fn parses_nested_infix_operators() {
    parses_to! {
        parser: MistQLParser,
        input: "1 + 2 * 3",
        rule: Rule::query,
        tokens: [
            number(0,1),
            infix_op(2,3),
            number(4,5),
            infix_op(6,7),
            number(8,9)
        ]
    }
}

#[test]
fn parses_infix_operators_as_function_args() {
    parses_to! {
        parser: MistQLParser,
        input: "map @ + 1 [1, 2, 3]",
        rule: Rule::query,
        tokens: [
            function(0,19, [
                ident(0,3),
                at(4,5),
                infix_op(6,7),
                number(8,9),
                array(10,19, [
                    number(11,12),
                    number(14,15),
                    number(17,18)
                ])
            ])
        ]
    }
}
