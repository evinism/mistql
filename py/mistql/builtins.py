from typing import List, Dict, Callable
from mistql.runtime_value import RuntimeValue, RuntimeValueType
from mistql.expression import Expression
from mistql.stack import Stack

Args = List[Expression]
Exec = Callable[[Expression, Stack], RuntimeValue]

FunctionDefinitionType = Callable[
    [Args, Stack, Exec],
    RuntimeValue,
]

def log(arguments: Args, stack: Stack, execute: Exec) -> RuntimeValue:
    if len(arguments) != 1:
        raise Exception("log takes one argument")
    res = execute(arguments[0], stack)
    print(res)
    return res


def reverse(arguments: Args, stack: Stack, execute: Exec) -> RuntimeValue:
    if len(arguments) != 1:
        raise Exception("reverse takes one argument")
    arg = execute(arguments[0], stack)
    if arg.type != RuntimeValueType.Array:
        raise Exception(f"reverse takes an array, got {arg}")
    return RuntimeValue.of(list(reversed(arg.value)))


def unary_minus(arguments: Args, stack: Stack, execute: Exec) -> RuntimeValue:
    if len(arguments) != 1:
        raise Exception("unary minus takes one argument")
    res = execute(arguments[0], stack)
    if res.type != RuntimeValueType.Number:
        raise Exception(f"unary_minus takes a number, got {res}")
    return RuntimeValue.of(-res.value)

def unary_not(arguments: Args, stack: Stack, execute: Exec) -> RuntimeValue:
    if len(arguments) != 1:
        raise Exception("unary not takes one argument")
    res = execute(arguments[0], stack)
    return RuntimeValue.of(not res.truthy())

def if_else(arguments: Args, stack: Stack, execute: Exec) -> RuntimeValue:
    if len(arguments) != 3:
        raise Exception("if takes three arguments")
    condition = execute(arguments[0], stack)
    if condition.truthy():
        return execute(arguments[1], stack)
    else:
        return execute(arguments[2], stack)

def add(arguments: Args, stack: Stack, execute: Exec) -> RuntimeValue:
    if len(arguments) != 2:
        raise Exception("add takes two arguments")
    left = execute(arguments[0], stack)
    right = execute(arguments[1], stack)
    if left.type != right.type:
        raise Exception(f"add: {left} and {right} are not the same type")
    if left.type == RuntimeValueType.Number:
        return RuntimeValue.of(left.value + right.value)
    if left.type == RuntimeValueType.String:
        return RuntimeValue.of(left.value + right.value)
    if left.type == RuntimeValueType.Array:
        return RuntimeValue.of(left.value + right.value)
    raise Exception(f"add: {left.type} is not supported")

def subtract(arguments: Args, stack: Stack, execute: Exec) -> RuntimeValue:
    if len(arguments) != 2:
        raise Exception("subtract takes two arguments")
    left = execute(arguments[0], stack)
    right = execute(arguments[1], stack)
    if left.type != RuntimeValueType.Number or right.type != RuntimeValueType.Number:
        raise Exception(f"subtract: {left} and {right} are not both numbers")
    return RuntimeValue.of(left.value - right.value)

def multiply(arguments: Args, stack: Stack, execute: Exec) -> RuntimeValue:
    if len(arguments) != 2:
        raise Exception("multiply takes two arguments")
    left = execute(arguments[0], stack)
    right = execute(arguments[1], stack)
    if left.type != RuntimeValueType.Number or right.type != RuntimeValueType.Number:
        raise Exception(f"multiply: {left} and {right} are not both numbers")
    return RuntimeValue.of(left.value * right.value)

def divide(arguments: Args, stack: Stack, execute: Exec) -> RuntimeValue:
    if len(arguments) != 2:
        raise Exception("divide takes two arguments")
    left = execute(arguments[0], stack)
    right = execute(arguments[1], stack)
    if left.type != RuntimeValueType.Number or right.type != RuntimeValueType.Number:
        raise Exception(f"divide: {left} and {right} are not both numbers")
    return RuntimeValue.of(left.value / right.value)

def mod(arguments: Args, stack: Stack, execute: Exec) -> RuntimeValue:
    if len(arguments) != 2:
        raise Exception("mod takes two arguments")
    left = execute(arguments[0], stack)
    right = execute(arguments[1], stack)
    if left.type != RuntimeValueType.Number or right.type != RuntimeValueType.Number:
        raise Exception(f"mod: {left} and {right} are not both numbers")
    return RuntimeValue.of(left.value % right.value)

def eq(arguments: Args, stack: Stack, execute: Exec) -> RuntimeValue:
    if len(arguments) != 2:
        raise Exception("eq takes two arguments")
    left = execute(arguments[0], stack)
    right = execute(arguments[1], stack)
    return RuntimeValue.of(RuntimeValue.eq(left, right))

def neq(arguments: Args, stack: Stack, execute: Exec) -> RuntimeValue:
    if len(arguments) != 2:
        raise Exception("neq takes two arguments")
    left = execute(arguments[0], stack)
    right = execute(arguments[1], stack)
    return RuntimeValue.of(not RuntimeValue.eq(left, right).value)

def and_fn(arguments: Args, stack: Stack, execute: Exec) -> RuntimeValue:
    if len(arguments) != 2:
        raise Exception("and takes two arguments")
    left = execute(arguments[0], stack)
    right = execute(arguments[1], stack)
    if left.truthy():
        return right
    else:
        return left

def or_fn(arguments: Args, stack: Stack, execute: Exec) -> RuntimeValue:
    if len(arguments) != 2:
        raise Exception("or takes two arguments")
    left = execute(arguments[0], stack)
    right = execute(arguments[1], stack)
    if left.truthy():
        return left
    else:
        return right

def count(arguments: Args, stack: Stack, execute: Exec) -> RuntimeValue:
    if len(arguments) != 1:
        raise Exception("count takes one argument")
    arg = execute(arguments[0], stack)
    if arg.type != RuntimeValueType.Array:
        raise Exception(f"count: {arg} is not an array")
    return RuntimeValue.of(len(arg.value))

def keys(arguments: Args, stack: Stack, execute: Exec) -> RuntimeValue:
    if len(arguments) != 1:
        raise Exception("keys takes one argument")
    arg = execute(arguments[0], stack)
    if arg.type != RuntimeValueType.Object:
        raise Exception(f"keys: {arg} is not an object")
    return RuntimeValue.of(list(arg.value.keys()))

builtins = {
    "-/unary": unary_minus,
    "!/unary": unary_not,
    "+": add,
    "-": subtract,
    "*": multiply,
    "/": divide,
    "%": mod,
    "==": eq,
    "!=": neq,
    "&&": and_fn,
    "||": or_fn,
    "count": count,
    "keys": keys,
    "if": if_else,
    "reverse": reverse,
    "log": log,
}
