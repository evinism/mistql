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
