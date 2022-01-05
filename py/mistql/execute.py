from typing import List, Dict, Callable
from mistql.runtime_value import RuntimeValue, RuntimeValueType
from mistql.expression import (
    Expression,
    RefExpression,
    FnExpression,
    ValueExpression,
    ArrayExpression,
    ObjectExpression,
    PipeExpression,
)

StackFrame = Dict[str, RuntimeValue]
Stack = List[StackFrame]


FunctionDefinitionType = Callable[
    [
        List[Expression],
        Stack,
        Callable[[Expression, Stack], RuntimeValue],
    ],
    RuntimeValue,
]


def execute_fncall(head, arguments, stack: Stack):
    fn = execute(head, stack)
    if fn.type != RuntimeValueType.Function:
        raise Exception(f"Tried to call a non-function: {fn}")
    # Not enforced, but definitely should be.
    function_definition: FunctionDefinitionType = fn.value
    return function_definition(arguments, stack, execute)


def execute(ast: Expression, stack: Stack) -> RuntimeValue:
    if isinstance(ast, ValueExpression):
        return ast.value
    elif isinstance(ast, RefExpression):
        return stack[-1][ast.name]
    elif isinstance(ast, FnExpression):
        return execute_fncall(ast.fn, ast.args, stack)
    elif isinstance(ast, ArrayExpression):
        return RuntimeValue.of([execute(item, stack) for item in ast.items])
    elif isinstance(ast, ObjectExpression):
        return RuntimeValue.of(
            {key: execute(value, stack) for key, value in ast.entries.items()}
        )
    raise NotImplementedError("execute() not implemented for " + ast.type)


def execute_outer(ast: Expression, data: RuntimeValue) -> RuntimeValue:
    stack: Stack = [
        {
            "@": data,
            "null": RuntimeValue.of(None),
            "true": RuntimeValue.of(True),
            "false": RuntimeValue.of(False),
        }
    ]
    return execute(ast, stack)
