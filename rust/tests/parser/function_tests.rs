use mistql::{MistQLParser, Rule};

const _GRAMMAR: &str = include_str!("../../src/mistql.pest");

#[test]
fn parses_basic_function_call() {
    parses_to! {
        parser: MistQLParser,
        input: "count [1,2,3]",
        rule: Rule::query,
        tokens: [
            function(0,13, [
                ident(0,5),
                array(6,13, [
                    number(7,8),
                    number(9,10),
                    number(11,12)
                ])
            ])
        ]
    }
}

#[test]
fn parses_function_with_three_arguments() {
    parses_to! {
        parser: MistQLParser,
        input: "if false 1 2",
        rule: Rule::query,
        tokens: [
            function(0,12, [
                ident(0,2),
                bool(3,8),
                number(9,10),
                number(11,12)
            ])
        ]
    }
}

// #[test]
// fn parses_function_with_override_name() {
//     parses_to! {
//         parser: MistQLParser,
//         input: "$.filter @ > 1 nums",
//         rule: Rule::query,
//         tokens: [
//             function(0,19, [
//                 ident(2,8),
//                 at(9,10),
//                 infix_op(11,12),
//                 number(13,14),
//                 ident(15,19)
//             ])
//         ]
//     }
// }

#[test]
fn functions_are_first_class_citizens() {
    parses_to! {
        parser: MistQLParser,
        input: "(if toggle keys values) {one: \"two\"}",
        rule: Rule::query,
        tokens: [
            function(0,36, [
                function(1,22, [
                    ident(1,3),
                    ident(4,10),
                    ident(11,15),
                    ident(16,22)
                ]),
                object(24,36, [
                    keyval(25,35, [
                        ident(25,28),
                        string(30,35, [
                            inner(31,34)
                        ])
                    ])
                ])
            ])
        ]
    }
}
