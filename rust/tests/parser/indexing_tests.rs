use mistql::{MistQLParser, Rule};

const _GRAMMAR: &str = include_str!("../../src/mistql.pest");

#[test]
fn parses_indexed_value() {
    parses_to! {
        parser: MistQLParser,
        input: "@[1]",
        rule: Rule::query,
        tokens: [
            indexed_value(0,4, [
                at(0,1),
                number(2,3)
            ])
        ]
    }
}

#[test]
fn parses_negative_indexed_value() {
    parses_to! {
        parser: MistQLParser,
        input: "@[-1]",
        rule: Rule::query,
        tokens: [
            indexed_value(0,5, [
                at(0,1),
                number(2,4)
            ])
        ]
    }
}

#[test]
fn parses_range_index() {
    parses_to! {
        parser: MistQLParser,
        input: "@[1:4]",
        rule: Rule::query,
        tokens: [
            indexed_value(0,6, [
                at(0,1),
                range(2,5, [
                    number(2,3),
                    number(4,5)
                ])
            ])
        ]
    }
}

#[test]
fn parses_range_index_with_no_start() {
    parses_to! {
        parser: MistQLParser,
        input: "@[:4]",
        rule: Rule::query,
        tokens: [
            indexed_value(0,5, [
                at(0,1),
                low_range(2,4, [
                    number(3,4)
                ])
            ])
        ]
    }
}

#[test]
fn parses_range_index_with_no_end() {
    parses_to! {
        parser: MistQLParser,
        input: "@[4:]",
        rule: Rule::query,
        tokens: [
            indexed_value(0,5, [
                at(0,1),
                high_range(2,4, [
                    number(2,3)
                ])
            ])
        ]
    }
}

// #[test]
// fn parses_selector() {
//     parses_to! {
//         parser: MistQLParser,
//         input: "one.two",
//         rule: Rule::query,
//         tokens: [
//             ident(0,3),
//             infix_op(3,4),
//             ident(4,7)
//         ]
//     }
// }

// #[test]
// fn parses_deep_selector() {
//     parses_to! {
//         parser: MistQLParser,
//         input: "one.two.three.four",
//         rule: Rule::query,
//         tokens: [
//             ident(0,3),
//             infix_op(3,4),
//             ident(4,7),
//             infix_op(7,8),
//             ident(8,13),
//             infix_op(13,14),
//             ident(14,18)
//         ]
//     }
// }

// #[test]
// fn parses_selector_with_lhs_expr() {
//     parses_to! {
//         parser: MistQLParser,
//         input: "(@ | apply @[0]).hello",
//         rule: Rule::query,
//         tokens: [
//             at(1,2),
//             infix_op(3,4),
//             function(5,15, [
//                 ident(5,10),
//                 subscript(11,15 ,[
//                     at(11,12),
//                     number(13,14)
//                 ])
//             ]),
//             infix_op(16,17),
//             ident(17,22)
//         ]
//     }
// }
