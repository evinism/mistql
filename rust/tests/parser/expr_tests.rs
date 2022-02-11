use mistql::parse::{MistQLParser, Rule};

const _GRAMMAR: &str = include_str!("../../src/mistql.pest");

#[test]
fn parses_piped_expressions() {
    parses_to! {
        parser: MistQLParser,
        // this won't evaluate but it should parse
        input: "null | true | [1,2,3]",
        rule: Rule::query,
        tokens: [
            piped_expr(0,21, [
                null(0,4),
                bool(7,11),
                array(14,21, [
                    number(15,16),
                    number(17,18),
                    number(19,20)
                ])
            ])
        ]
    }
}
