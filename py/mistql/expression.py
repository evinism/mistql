from enum import Enum
from typing import Dict, List, Union
from mistql.runtime_value import RuntimeValue
import json


class ExpressionType(Enum):
    Fncall = "fncall"
    Reference = "reference"
    Literal = "literal"
    Value = "value"
    Array = "array"
    Object = "object"
    Pipe = "pipe"


class BaseExpression:
    """Represents the MistQL expression, after parsing"""

    @classmethod
    def from_lark(cls, lark_tree):
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
                [cls.from_lark(child) for child in lark_tree.children]
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
                    child.children[0].value: cls.from_lark(child.children[2])
                    for child in lark_tree.children
                }
            )
        elif lark_tree.data == "pipe":
            return PipeExpression(
                [cls.from_lark(child) for child in lark_tree.children]
            )
        elif lark_tree.data == "fncall":
            return FnExpression(
                cls.from_lark(lark_tree.children[0]),
                [
                    cls.from_lark(child)
                    for child in lark_tree.children[1:]
                    if getattr(child, "type", None) != "WS"
                ],
            )
        elif lark_tree.data == "neg":
            return FnExpression(
                RefExpression("-/unary"),
                [
                    cls.from_lark(child)
                    for child in lark_tree.children[1:]
                    if getattr(child, "type", None) != "WS"
                ],
            )
        elif lark_tree.data == "not":
            return FnExpression(
                RefExpression("!/unary"),
                [
                    cls.from_lark(child)
                    for child in lark_tree.children[1:]
                    if getattr(child, "type", None) != "WS"
                ],
            )
        else:
            raise Exception(f"Unknown lark expression type: {lark_tree.data}")

    def __init__(self, type: ExpressionType):
        self.type = type


class FnExpression(BaseExpression):
    def __init__(self, fn: BaseExpression, args: List[BaseExpression]):
        super().__init__(ExpressionType.Fncall)
        self.fn = fn
        self.args = args


class RefExpression(BaseExpression):
    def __init__(self, name: str):
        super().__init__(ExpressionType.Reference)
        self.name = name


class ValueExpression(BaseExpression):
    def __init__(self, value: RuntimeValue):
        super().__init__(ExpressionType.Value)
        self.value = value


class ArrayExpression(BaseExpression):
    def __init__(self, items: List[BaseExpression]):
        super().__init__(ExpressionType.Array)
        self.items = items


class ObjectExpression(BaseExpression):
    def __init__(self, entries: Dict[str, BaseExpression]):
        super().__init__(ExpressionType.Object)
        self.entries = entries


class PipeExpression(BaseExpression):
    def __init__(self, stages: List[BaseExpression]):
        super().__init__(ExpressionType.Pipe)
        self.stages = stages


Expression = Union[
    FnExpression,
    RefExpression,
    ValueExpression,
    ArrayExpression,
    ObjectExpression,
    PipeExpression,
]
