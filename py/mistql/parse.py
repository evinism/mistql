from enum import Enum
from lark import Lark
from mistql.runtime_value import RuntimeValue
from mistql.expression import BaseExpression
from typing import List

mistql_parser = Lark(
    r"""
?start: piped_expression

?piped_expression: simple_expression
    | simple_expression ("|" simple_expression)+ -> pipe
?simple_expression : fncall | op_a
?simplevalue: reference | literal | "(" piped_expression ")"
?fncall: op_a (WS op_a)+ -> fncall


namedref: NAME -> namedref
at: "@" -> at
dollar: "$" -> dollar

?reference: namedref | at | dollar
?literal: object         -> object
    | array              -> array
    | string             -> string
    | NUMBER             -> number
    | "true"             -> true
    | "false"            -> false
    | "null"             -> null

array  : "[" [simple_expression ("," simple_expression)*] "]"
object : "{" [pair ("," pair)*] "}"
pair   : (string | NAME) ":" simple_expression
string : ESCAPED_STRING

indexing:  "[" piped_expression (":" piped_expression?)* "]"

?op_a: op_b
    | op_a "||" op_b -> or

?op_b: op_c
    | op_b "&&" op_c -> and

?op_c: op_d
    | op_c "==" op_d -> eq
    | op_c "!=" op_d -> neq
    | op_c "=~" op_d -> match

?op_d: op_e
    | op_d ">" op_e -> gt
    | op_d "<" op_e -> lt
    | op_d ">=" op_e -> gte
    | op_d "<=" op_e -> lte

?op_e: op_f
    | op_e "+" op_f -> plus
    | op_e "-" op_f -> minus

?op_f: op_g
    | op_f "*" op_g -> mul
    | op_f "/" op_g -> div
    | op_f "%" op_g -> mod

?op_g: op_h
    | "!" op_g -> not
    | "-" op_g -> neg

?op_h: simplevalue
    | op_h "." reference -> dot
    | op_h indexing -> index

%import common.ESCAPED_STRING
%import common.WS
%import common.CNAME -> NAME
%import common.NUMBER

%ignore WS


""",
    parser="earley",
)


def parse(raw):
    parsed = mistql_parser.parse(raw)
    print(parsed.pretty())
    return BaseExpression.from_lark(parsed)
