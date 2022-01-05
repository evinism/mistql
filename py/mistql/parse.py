from enum import Enum
from lark import Lark
from mistql.runtime_value import RuntimeValue
from mistql.expression import BaseExpression, RefExpression, FnExpression, ValueExpression, ArrayExpression, ObjectExpression, PipeExpression
from typing import List
import json

mistql_parser = Lark(
    r"""
?start: piped_expression

?piped_expression: simple_expression
    | simple_expression ("|" simple_expression)+ -> pipe
?simple_expression : fncall | op_a
?simplevalue: literal | reference | "(" piped_expression ")"
?fncall: op_a (WS op_a)+ -> fncall


namedref: NAME -> namedref
at: "@" -> at
dollar: "$" -> dollar

?reference: namedref | at | dollar
?literal: object
    | array              -> array
    | ESCAPED_STRING     -> string
    | NUMBER             -> number
    | "true"             -> true
    | "false"            -> false
    | "null"             -> null

array  : "[" [simple_expression ("," simple_expression)*] "]"
object : "{" [pair ("," pair)*] "}"
pair   : (ESCAPED_STRING | NAME) ":" simple_expression

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

def from_lark(lark_tree):
    if lark_tree.data == "namedref":
        return RefExpression(lark_tree.children[0])
    elif lark_tree.data == "at":
        return RefExpression("@")
    elif lark_tree.data == "dollar":
        return RefExpression("$")
    elif lark_tree.data == "number":
        return ValueExpression(RuntimeValue.of(float(lark_tree.children[0].value)))
    elif lark_tree.data == "string":
        value = json.loads(lark_tree.children[0].value)
        return ValueExpression(RuntimeValue.of(value))
    elif lark_tree.data == "array":
        return ArrayExpression(
            [from_lark(child) for child in lark_tree.children]
        )
    elif lark_tree.data == "true":
        return ValueExpression(RuntimeValue.of(True))
    elif lark_tree.data == "false":
        return ValueExpression(RuntimeValue.of(False))
    elif lark_tree.data == "null":
        return ValueExpression(RuntimeValue.of(None))
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
                for child in lark_tree.children[1:]
            ],
        )
    else:
        raise Exception(f"Unknown lark expression type: {lark_tree.data}")



def parse(raw):
    parsed = mistql_parser.parse(raw)
    print(parsed.pretty())
    return from_lark(parsed)
