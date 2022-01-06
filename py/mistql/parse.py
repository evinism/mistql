from enum import Enum
from lark import Lark, Tree, Token
from mistql.expression import (
    RefExpression,
    FnExpression,
    ValueExpression,
    ArrayExpression,
    ObjectExpression,
    PipeExpression,
)
from typing import Union, List
import json

from mistql.expression import Expression

mistql_parser = Lark(
    r"""

_W: WS
AT: "@"
DOLLAR: "$"
TRUE: "true"
FALSE: "false"
NULL: "null"

_wsl{param}: _W? param
_wsr{param}: param _W?
_wslr{param}: _W? param _W?

?start: piped_expression

?piped_expression: simple_expression
    | simple_expression ("|" simple_expression)+ -> pipe
?simple_expression : _wslr{op_a} | _wslr{fncall}
?simplevalue: literal | reference | _wsr{"("} piped_expression _wsl{")"}
?fncall: op_a (_W op_a)+ -> fncall

?reference: CNAME | AT | DOLLAR
?literal: object
    | array
    | ESCAPED_STRING
    | NUMBER
    | TRUE
    | FALSE
    | NULL

array  : _wsr{"["} (simple_expression (_wslr{","} simple_expression)*)? _wsl{"]"} -> array
object : _wsr{"{"} (object_entry (_wslr{","} object_entry)*)? _wsl{"}"} -> object
object_entry : (ESCAPED_STRING | CNAME) _wslr{":"} simple_expression -> object_entry

WCOLON: WS? ":" WS?

?indexing:  _wsr{"["} index_innards _wsl{"]"} -> indexing
!index_innards: piped_expression? (WCOLON piped_expression?)* -> index_innards


?op_a: op_b
    | op_a _wslr{"||"} op_b -> or

?op_b: op_c
    | op_b _wslr{"&&"} op_c -> and

?op_c: op_d
    | op_c _wslr{"=="} op_d -> eq
    | op_c _wslr{"!="} op_d -> neq
    | op_c _wslr{"=~"} op_d -> match

?op_d: op_e
    | op_d _wslr{">"} op_e -> gt
    | op_d _wslr{"<"} op_e -> lt
    | op_d _wslr{">="} op_e -> gte
    | op_d _wslr{"<="} op_e -> lte

?op_e: op_f
    | op_e _wslr{"+"} op_f -> plus
    | op_e _wslr{"-"} op_f -> minus

?op_f: op_g
    | op_f _wslr{"*"} op_g -> mul
    | op_f _wslr{"/"} op_g -> div
    | op_f _wslr{"%"} op_g -> mod

?op_g: op_h
    | _wsr{"!"} op_g -> not
    | _wsr{"-"} op_g -> neg

?op_h: simplevalue
    | op_h _wslr{"."} reference -> dot
    | op_h indexing -> index

%import common.ESCAPED_STRING
%import common.WS
%import common.CNAME
%import common.NUMBER

""",
    parser="earley",
)

function_mappings = {
    "dot": ".",
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


def process_lark_tree(lark_node: Tree) -> Expression:
    if lark_node.data == "array":
        return ArrayExpression([from_lark(child) for child in lark_node.children])
    elif lark_node.data == "object":
        obj = {}
        for child in lark_node.children:
            key = from_lark(child.children[0])
            if isinstance(key, ValueExpression):
                key = key.value.value
            elif isinstance(key, RefExpression):
                key = key.name
            else:
                raise Exception(f"Unknown key type {type(key)}")
            value = from_lark(child.children[1])
            obj[key] = value
        return ObjectExpression(obj)
    elif lark_node.data == "pipe":
        return PipeExpression([from_lark(child) for child in lark_node.children])
    elif lark_node.data == "fncall":
        return FnExpression(
            from_lark(lark_node.children[0]),
            [from_lark(child) for child in lark_node.children[1:]],
        )
    elif lark_node.data in function_mappings:
        return FnExpression(
            RefExpression(function_mappings[lark_node.data]),
            [from_lark(child) for child in lark_node.children[:]],
        )
    elif lark_node.data == "index":
        # This is gross becase i can't figure out how to get the tree to look a little more
        # sensible.
        base, indexing = lark_node.children
        innards = indexing.children[0]
        fnexp_args: List[Expression] = []
        prev_was_token = True
        for child in innards.children:
            if isinstance(child, Token) and child.value == ":":
                if prev_was_token:
                    fnexp_args.append(ValueExpression.of(None))
                prev_was_token = True
                continue
            else:
                fnexp_args.append(from_lark(child))
                prev_was_token = False
        if prev_was_token:
            fnexp_args.append(ValueExpression.of(None))
        fnexp_args.append(from_lark(base))
        return FnExpression(RefExpression("index"), fnexp_args)
    else:
        raise Exception(f"Unknown lark expression type: {lark_node.data}")


def process_lark_token(lark_node: Token) -> Expression:
    if lark_node.type == "NUMBER":
        return ValueExpression.of(float(lark_node.value))
    elif lark_node.type == "ESCAPED_STRING":
        value = json.loads(lark_node.value)
        return ValueExpression.of(value)
    elif lark_node.type == "TRUE":
        return ValueExpression.of(True)
    elif lark_node.type == "FALSE":
        return ValueExpression.of(False)
    elif lark_node.type == "NULL":
        return ValueExpression.of(None)
    elif lark_node.type == "CNAME":
        return RefExpression(lark_node.value)
    elif lark_node.type == "AT":
        return RefExpression("@")
    elif lark_node.type == "DOLLAR":
        return RefExpression("$")
    else:
        raise Exception(f"Unknown token type {lark_node.type}")


def from_lark(lark_node: Union[Tree, Token]):
    if isinstance(lark_node, Token):
        return process_lark_token(lark_node)
    elif isinstance(lark_node, Tree):
        return process_lark_tree(lark_node)
    else:
        raise Exception(f"Unknown lark node type: {type(lark_node)}")


def parse(raw):
    parsed = mistql_parser.parse(raw)
    return from_lark(parsed)
