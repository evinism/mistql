from typing import List, Dict, Callable
from mistql.runtime_value import RuntimeValue, RuntimeValueType
from mistql.expression import Expression
from mistql.stack import Stack
from mistql.expression import RefExpression
from mistql.stack import add_runtime_value_to_stack
import re

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
    return RuntimeValue.of(arg.keys())


def dot(arguments: Args, stack: Stack, execute: Exec) -> RuntimeValue:
    if len(arguments) != 2:
        raise Exception("dot takes two arguments")
    left = execute(arguments[0], stack)
    right = arguments[1]
    if not isinstance(right, RefExpression):
        raise Exception(f"dot: rhs is not a ref")
    return left.access(right.name)


def map(arguments: Args, stack: Stack, execute: Exec) -> RuntimeValue:
    if len(arguments) != 2:
        raise Exception("map takes two arguments")
    mutation = arguments[0]
    operand = execute(arguments[1], stack)
    if operand.type != RuntimeValueType.Array:
        raise Exception(f"map: {operand} is not an array")
    out: List[RuntimeValue] = []
    for item in operand.value:
        res = execute(mutation, add_runtime_value_to_stack(item, stack))
        out.append(res)
    return RuntimeValue.of(out)


def filter(arguments: Args, stack: Stack, execute: Exec) -> RuntimeValue:
    if len(arguments) != 2:
        raise Exception("filter takes two arguments")
    mutation = arguments[0]
    operand = execute(arguments[1], stack)
    if operand.type != RuntimeValueType.Array:
        raise Exception(f"filter: {operand} is not an array")
    out: List[RuntimeValue] = []
    for item in operand.value:
        res = execute(mutation, add_runtime_value_to_stack(item, stack))
        if res.truthy():
            out.append(item)
    return RuntimeValue.of(out)


def mapvalues(arguments: Args, stack: Stack, execute: Exec) -> RuntimeValue:
    if len(arguments) != 2:
        raise Exception("mapvalues takes two arguments")
    mutation = arguments[0]
    operand = execute(arguments[1], stack)
    if operand.type != RuntimeValueType.Object:
        raise Exception(f"mapvalues: {operand} is not an object")
    out: Dict[str, RuntimeValue] = {}
    for key, value in operand.value.items():
        res = execute(mutation, add_runtime_value_to_stack(value, stack))
        out[key] = res
    return RuntimeValue.of(out)


def mapkeys(arguments: Args, stack: Stack, execute: Exec) -> RuntimeValue:
    if len(arguments) != 2:
        raise Exception("mapkeys takes two arguments")
    mutation = arguments[0]
    operand = execute(arguments[1], stack)
    if operand.type != RuntimeValueType.Object:
        raise Exception(f"mapkeys: {operand} is not an object")
    out: Dict[str, RuntimeValue] = {}
    for key, value in operand.value.items():
        res = execute(mutation, add_runtime_value_to_stack(RuntimeValue.of(key), stack))
        if res.type != RuntimeValueType.String:
            raise Exception(f"mapkeys: {res} is not a string")
        out[res.value] = value
    return RuntimeValue.of(out)


def _index_double(
    operand: RuntimeValue,
    index_one: RuntimeValue,
    index_two: RuntimeValue,
):
    if operand.type != RuntimeValueType.Array:
        raise Exception(f"index: {operand} is not an array")
    pass


def _index_single(operand: RuntimeValue, index: RuntimeValue):
    if (
        operand.type == RuntimeValueType.Array
        or operand.type == RuntimeValueType.String
    ):
        if index.type != RuntimeValueType.Number:
            raise Exception(f"index: Non-numbers cannot be used on arrays")
        index_num = index.value
        if index_num % 1 != 0:
            raise Exception(f"index: Non-integers cannot be used on arrays")
        if index_num < 0:
            index_num = len(operand.value) + index_num
        if index_num < 0 or index_num >= len(operand.value):
            return RuntimeValue.of(None)
        return operand.value[index_num]
    else:
        return operand.access(index.to_string())


def index(arguments: Args, stack: Stack, execute: Exec) -> RuntimeValue:
    if len(arguments) not in {2, 3}:
        raise Exception("index takes two to three arguments")
    operand = execute(arguments[0], stack)
    if len(arguments) == 3:
        return _index_double(
            operand, execute(arguments[1], stack), execute(arguments[2], stack)
        )
    else:
        return _index_single(operand, execute(arguments[1], stack))


def string(arguments: Args, stack: Stack, execute: Exec) -> RuntimeValue:
    if len(arguments) != 1:
        raise Exception("string takes one argument")
    arg = execute(arguments[0], stack)
    return RuntimeValue.of(arg.to_string())


def float(arguments: Args, stack: Stack, execute: Exec) -> RuntimeValue:
    if len(arguments) != 1:
        raise Exception("float takes one argument")
    arg = execute(arguments[0], stack)
    return RuntimeValue.of(arg.to_float())


def regex(arguments: Args, stack: Stack, execute: Exec) -> RuntimeValue:
    if len(arguments) not in {1, 2}:
        raise Exception("regex takes one to two arguments")
    pattern = execute(arguments[0], stack)
    if pattern.type != RuntimeValueType.String:
        raise Exception(f"regex: {pattern} is not a string")

    if len(arguments) == 2:
        # TODO: flags
        flags = execute(arguments[1], stack)
        if flags.type != RuntimeValueType.String:
            raise Exception(f"regex: {flags} is not a string")
    else:
        flags = RuntimeValue.of("")
    return RuntimeValue(RuntimeValueType.Regex, re.compile(pattern.value, flags=0))


builtins = {
    ".": dot,
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
    "float": float,
    "map": map,
    "filter": filter,
    "index": index,
    "string": string,
    "mapvalues": mapvalues,
    "mapkeys": mapkeys,
    "if": if_else,
    "reverse": reverse,
    "regex": regex,
    "log": log,
}
