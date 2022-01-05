from enum import Enum
from lark import Lark, Tree, Token
from mistql.runtime_value import RuntimeValue
from mistql.expression import BaseExpression, RefExpression, FnExpression, ValueExpression, ArrayExpression, ObjectExpression, PipeExpression
from typing import List, Union
import json

mistql_parser = Lark(
    r"""
?start: piped_expression

?piped_expression: simple_expression
    | simple_expression ("|" simple_expression)+ -> pipe
?simple_expression : fncall | op_a
?simplevalue: literal | reference | "(" piped_expression ")"
?fncall: op_a (WS op_a)+ -> fncall

?reference: NAME | AT | DOLLAR
?literal: object
    | array
    | ESCAPED_STRING
    | NUMBER
    | TRUE
    | FALSE
    | NULL

AT: "@"
DOLLAR: "$"
TRUE: "true"
FALSE: "false"
NULL: "null"

array  : "[" [simple_expression ("," simple_expression)*] "]" -> array
object : "{" [object_entry ("," object_entry)*] "}" -> object
object_entry   : (ESCAPED_STRING | NAME) ":" simple_expression -> object_entry

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

function_mappings = {
    "neg": "-/unary",
    "not": "!/unary",
    "gt": ">",
    "lt": "<",
    "gte": ">=",
    "lte": "<=",
    "eq": "==",
    "neq": "!=",
    "match": "=~",
    "plus": "+",
    "minus": "-",
    "mul": "*",
    "div": "/",
    "mod": "%",
    "and": "&&",
    "or": "||",
}

def from_lark(lark_tree: Union[Tree, Token]):
    if isinstance(lark_tree, Token):
        if lark_tree.type == "NUMBER":
            return ValueExpression(RuntimeValue.of(float(lark_tree.value)))
        elif lark_tree.type == "ESCAPED_STRING":
            value = json.loads(lark_tree.value)
            return ValueExpression(RuntimeValue.of(value))
        elif lark_tree.type == "TRUE":
            return ValueExpression(RuntimeValue.of(True))
        elif lark_tree.type == "FALSE":
            return ValueExpression(RuntimeValue.of(False))
        elif lark_tree.type == "NULL":
            return ValueExpression(RuntimeValue.of(None))
        elif lark_tree.type == "NAME":
            return RefExpression(lark_tree.value)
        elif lark_tree.type == "AT":
            return RefExpression("@")
        elif lark_tree.type == "DOLLAR":
            return RefExpression("$")
        else:
            raise Exception(f"Unknown token type {lark_tree.type}")
    else:
        if lark_tree.data == "array":
            return ArrayExpression(
                [from_lark(child) for child in lark_tree.children]
            )
        elif lark_tree.data == "object":
            return ObjectExpression(
                {
                    child.children[0].value: from_lark(child.children[2])
                    for child in lark_tree.children
                }
            )
        elif lark_tree.data == "pipe":
            return PipeExpression(
                [from_lark(child) for child in lark_tree.children]
            )
        elif lark_tree.data == "fncall":
            return FnExpression(
                from_lark(lark_tree.children[0]),
                [
                    from_lark(child)
                    for child in lark_tree.children[1:]
                    if getattr(child, "type", None) != "WS"
                ],
            )
        elif lark_tree.data in function_mappings:
            return FnExpression(
                RefExpression(function_mappings[lark_tree.data]),
                [
                    from_lark(child)
                    for child in lark_tree.children[:]
                ],
            )
        else:
            raise Exception(f"Unknown lark expression type: {lark_tree.data}")


def parse(raw):
    parsed = mistql_parser.parse(raw)
    return from_lark(parsed)
