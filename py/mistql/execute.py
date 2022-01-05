from typing import List, Callable
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
from mistql.stack import Stack
from mistql.builtins import FunctionDefinitionType, builtins


def execute_fncall(head, arguments, stack: Stack):
    fn = execute(head, stack)
    if fn.type != RuntimeValueType.Function:
        raise Exception(f"Tried to call a non-function: {fn}")
    # Not enforced, but definitely should be.
    function_definition: FunctionDefinitionType = fn.value
    return function_definition(arguments, stack, execute)


def execute_pipe(stages: List[Expression], stack: Stack) -> RuntimeValue:
    for stage in stages:
        new_stack = stack.copy()
        data = execute(stage, stack)
        new_stack.append(
            {
                "@": data,
            }
        )
    return data


def find_in_stack(stack: Stack, name: str) -> RuntimeValue:
    for frame in reversed(stack):
        if name in frame:
            return frame[name]
    raise Exception(f"Could not find {name} in stack")


def execute(ast: Expression, stack: Stack) -> RuntimeValue:
    if isinstance(ast, ValueExpression):
        return ast.value
    elif isinstance(ast, RefExpression):
        return find_in_stack(stack, ast.name)
    elif isinstance(ast, FnExpression):
        return execute_fncall(ast.fn, ast.args, stack)
    elif isinstance(ast, ArrayExpression):
        return RuntimeValue.of([execute(item, stack) for item in ast.items])
    elif isinstance(ast, ObjectExpression):
        return RuntimeValue.of(
            {key: execute(value, stack) for key, value in ast.entries.items()}
        )
    elif isinstance(ast, PipeExpression):
        return execute_pipe(ast.stages, stack)
    raise NotImplementedError("execute() not implemented for " + ast.type)


def execute_outer(ast: Expression, data: RuntimeValue) -> RuntimeValue:
    top_stack_entry = {
        "@": data,
    }
    if data.type == RuntimeValueType.Object:
        for key, value in data.value.items():
            top_stack_entry[key] = value
    for builtin in builtins:
        top_stack_entry[builtin] = RuntimeValue.create_function(builtins[builtin])
    stack: Stack = [top_stack_entry]
    return execute(ast, stack)
