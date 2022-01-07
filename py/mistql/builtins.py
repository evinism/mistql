from typing import List, Dict, Callable, Tuple, Union
from mistql.runtime_value import RuntimeValue, RuntimeValueType
from mistql.expression import Expression
from mistql.stack import Stack
from mistql.expression import RefExpression
from mistql.stack import add_runtime_value_to_stack
import re
from functools import cmp_to_key
import statistics

Args = List[Expression]
Exec = Callable[[Expression, Stack], RuntimeValue]

FunctionDefinitionType = Callable[
    [Args, Stack, Exec],
    RuntimeValue,
]

builtins: Dict[str, FunctionDefinitionType] = {}


def builtin(name: str, min_args: int, max_args: Union[None, int] = None):
    if max_args is None:
        max_args = min_args

    def builtin_decorator(fn: FunctionDefinitionType) -> FunctionDefinitionType:
        def wrapped(arguments: Args, stack: Stack, exec: Exec):
            if not min_args < 0 and len(arguments) < min_args:
                raise Exception(f"{name} takes at least {min_args} arguments")
            if not max_args < 0 and len(arguments) > max_args:
                raise Exception(f"{name} takes at most {max_args} arguments")
            return fn(arguments, stack, exec)

        builtins[name] = wrapped
        return wrapped

    return builtin_decorator


@builtin("log", 1)
def log(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    res = exec(arguments[0], stack)
    print(res)
    return res


@builtin("reverse", 1)
def reverse(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    arg = exec(arguments[0], stack)
    if arg.type != RuntimeValueType.Array:
        raise Exception(f"reverse takes an array, got {arg}")
    return RuntimeValue.of(list(reversed(arg.value)))


@builtin("-/unary", 1)
def unary_minus(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    res = exec(arguments[0], stack)
    if res.type != RuntimeValueType.Number:
        raise Exception(f"unary_minus takes a number, got {res}")
    return RuntimeValue.of(-res.value)


@builtin("!/unary", 1)
def unary_not(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    res = exec(arguments[0], stack)
    return RuntimeValue.of(not res)


@builtin("if", 3)
def if_else(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    if exec(arguments[0], stack):
        return exec(arguments[1], stack)
    else:
        return exec(arguments[2], stack)


@builtin("+", 2)
def add(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    left = exec(arguments[0], stack)
    right = exec(arguments[1], stack)
    if left.type != right.type:
        raise Exception(f"add: {left} and {right} are not the same type")
    if left.type in {
        RuntimeValueType.Number,
        RuntimeValueType.String,
        RuntimeValueType.Array,
    }:
        return RuntimeValue.of(left.value + right.value)
    raise Exception(f"add: {left.type} is not supported")


@builtin("-", 2)
def subtract(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    left = exec(arguments[0], stack)
    right = exec(arguments[1], stack)
    if left.type != RuntimeValueType.Number or right.type != RuntimeValueType.Number:
        raise Exception(f"subtract: {left} and {right} are not both numbers")
    return RuntimeValue.of(left.value - right.value)


@builtin("*", 2)
def multiply(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    left = exec(arguments[0], stack)
    right = exec(arguments[1], stack)
    if left.type != RuntimeValueType.Number or right.type != RuntimeValueType.Number:
        raise Exception(f"multiply: {left} and {right} are not both numbers")
    return RuntimeValue.of(left.value * right.value)


@builtin("/", 2)
def divide(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    left = exec(arguments[0], stack)
    right = exec(arguments[1], stack)
    if left.type != RuntimeValueType.Number or right.type != RuntimeValueType.Number:
        raise Exception(f"divide: {left} and {right} are not both numbers")
    return RuntimeValue.of(left.value / right.value)


@builtin("%", 2)
def mod(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    left = exec(arguments[0], stack)
    right = exec(arguments[1], stack)
    if left.type != RuntimeValueType.Number or right.type != RuntimeValueType.Number:
        raise Exception(f"mod: {left} and {right} are not both numbers")
    return RuntimeValue.of(left.value % right.value)


@builtin("==", 2)
def eq(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    return exec(arguments[0], stack) == exec(arguments[1], stack)


@builtin("!=", 2)
def neq(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    return exec(arguments[0], stack) != exec(arguments[1], stack)


@builtin("&&", 2)
def and_fn(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    left = exec(arguments[0], stack)
    right = exec(arguments[1], stack)
    if left:
        return right
    else:
        return left


@builtin("||", 2)
def or_fn(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    left = exec(arguments[0], stack)
    right = exec(arguments[1], stack)
    if left:
        return left
    else:
        return right


@builtin("count", 1)
def count(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    arg = exec(arguments[0], stack)
    if arg.type != RuntimeValueType.Array:
        raise Exception(f"count: {arg} is not an array")
    return RuntimeValue.of(len(arg.value))


@builtin("keys", 1)
def keys(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    return RuntimeValue.of(exec(arguments[0], stack).keys())


@builtin(".", 2)
def dot(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    left = exec(arguments[0], stack)
    right = arguments[1]
    if not isinstance(right, RefExpression):
        raise Exception(f"dot: rhs is not a ref")
    return left.access(right.name)


@builtin("map", 2)
def map(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    mutation = arguments[0]
    operand = exec(arguments[1], stack)
    if operand.type != RuntimeValueType.Array:
        raise Exception(f"map: {operand} is not an array")
    out: List[RuntimeValue] = []
    for item in operand.value:
        res = exec(mutation, add_runtime_value_to_stack(item, stack))
        out.append(res)
    return RuntimeValue.of(out)


@builtin("reduce", 3)
def reduce(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    mutation = arguments[0]
    initial = exec(arguments[1], stack)
    operand = exec(arguments[2], stack)
    if operand.type != RuntimeValueType.Array:
        raise Exception(f"reduce: {operand} is not an array")
    out = initial
    for item in operand.value:
        acc_cur = RuntimeValue.of([out, item])
        out = exec(mutation, add_runtime_value_to_stack(acc_cur, stack))
    return out


@builtin("filter", 2)
def filter(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    mutation = arguments[0]
    operand = exec(arguments[1], stack)
    if operand.type != RuntimeValueType.Array:
        raise Exception(f"filter: {operand} is not an array")
    out: List[RuntimeValue] = []
    for item in operand.value:
        res = exec(mutation, add_runtime_value_to_stack(item, stack))
        if res:
            out.append(item)
    return RuntimeValue.of(out)


@builtin("mapvalues", 2)
def mapvalues(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    mutation = arguments[0]
    operand = exec(arguments[1], stack)
    if operand.type != RuntimeValueType.Object:
        raise Exception(f"mapvalues: {operand} is not an object")
    out: Dict[str, RuntimeValue] = {}
    for key, value in operand.value.items():
        res = exec(mutation, add_runtime_value_to_stack(value, stack))
        out[key] = res
    return RuntimeValue.of(out)


@builtin("mapkeys", 2)
def mapkeys(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    mutation = arguments[0]
    operand = exec(arguments[1], stack)
    if operand.type != RuntimeValueType.Object:
        raise Exception(f"mapkeys: {operand} is not an object")
    out: Dict[str, RuntimeValue] = {}
    for key, value in operand.value.items():
        res = exec(mutation, add_runtime_value_to_stack(RuntimeValue.of(key), stack))
        if res.type != RuntimeValueType.String:
            raise Exception(f"mapkeys: {res} is not a string")
        out[res.value] = value
    return RuntimeValue.of(out)


@builtin("filtervalues", 2)
def filtervalues(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    mutation = arguments[0]
    operand = exec(arguments[1], stack)
    if operand.type != RuntimeValueType.Object:
        raise Exception(f"filtervalues: {operand} is not an object")
    out: Dict[str, RuntimeValue] = {}
    for key, value in operand.value.items():
        res = exec(mutation, add_runtime_value_to_stack(value, stack))
        if res:
            out[key] = value
    return RuntimeValue.of(out)


@builtin("filterkeys", 2)
def filterkeys(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    mutation = arguments[0]
    operand = exec(arguments[1], stack)
    if operand.type != RuntimeValueType.Object:
        raise Exception(f"filterkeys: {operand} is not an object")
    out: Dict[str, RuntimeValue] = {}
    for key, value in operand.value.items():
        res = exec(mutation, add_runtime_value_to_stack(RuntimeValue.of(key), stack))
        if res:
            out[key] = value
    return RuntimeValue.of(out)


@builtin("find", 2)
def find(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    mutation = arguments[0]
    operand = exec(arguments[1], stack)
    if operand.type != RuntimeValueType.Array:
        raise Exception(f"find: {operand} is not an array")
    for item in operand.value:
        res = exec(mutation, add_runtime_value_to_stack(item, stack))
        if res:
            return item
    return RuntimeValue.of(None)


@builtin("apply", 2)
def apply(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    target = exec(arguments[1], stack)
    return exec(arguments[0], add_runtime_value_to_stack(target, stack))


def _index_double(
    index_one: RuntimeValue,
    index_two: RuntimeValue,
    operand: RuntimeValue,
):
    if (
        operand.type == RuntimeValueType.Array
        or operand.type == RuntimeValueType.String
    ):
        if index_one.type == RuntimeValueType.Null:
            index_one = RuntimeValue.of(0)
        if index_two.type == RuntimeValueType.Null:
            # This is terrible. I'm sorry.
            index_two = RuntimeValue.of(10000000000000)
        if (
            index_one.type != RuntimeValueType.Number
            or index_two.type != RuntimeValueType.Number
        ):
            raise Exception(f"index: Non-numbers cannot be used on arrays")
        index_one_num = index_one.value
        if index_one_num % 1 != 0:
            raise Exception(f"index: Non-integers cannot be used on arrays")
        if index_one_num < 0:
            index_one_num = len(operand.value) + index_one_num
        index_two_num = index_two.value
        if index_two_num % 1 != 0:
            raise Exception(f"index: Non-integers cannot be used on arrays")
        if index_two_num < 0:
            index_two_num = len(operand.value) + index_two_num
        return RuntimeValue.of(operand.value[int(index_one_num) : int(index_two_num)])
    else:
        raise Exception(f"index: {operand} is not an array or string")


def _index_single(index: RuntimeValue, operand: RuntimeValue):
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
        return RuntimeValue.of(operand.value[int(index_num)])
    else:
        return operand.access(index.to_string())


@builtin("index", 2, 3)
def index(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    if len(arguments) == 3:
        return _index_double(
            exec(arguments[0], stack),
            exec(arguments[1], stack),
            exec(arguments[2], stack),
        )
    else:
        return _index_single(exec(arguments[0], stack), exec(arguments[1], stack))


@builtin("string", 1)
def string(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    return RuntimeValue.of(exec(arguments[0], stack).to_string())


@builtin("float", 1)
def float(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    return RuntimeValue.of(exec(arguments[0], stack).to_float())


@builtin("regex", 1, 2)
def regex(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    pattern = exec(arguments[0], stack)
    if pattern.type != RuntimeValueType.String:
        raise Exception(f"regex: {pattern} is not a string")

    if len(arguments) == 2:
        flags = exec(arguments[1], stack)
        if flags.type != RuntimeValueType.String:
            raise Exception(f"regex: {flags} is not a string")
    else:
        flags = RuntimeValue.of("")

    flags_int = 0
    # Supported flags are /gims/
    if flags.value.find("i") != -1:
        flags_int |= re.IGNORECASE
    if flags.value.find("m") != -1:
        flags_int |= re.MULTILINE
    if flags.value.find("s") != -1:
        flags_int |= re.DOTALL

    # This is because python doesn't have a global flag.
    if flags.value.find("g") != -1:
        is_global = True
    else:
        is_global = False

    return RuntimeValue(
        RuntimeValueType.Regex,
        re.compile(pattern.value, flags=flags_int),
        modifiers={"global": is_global},
    )


@builtin("stringjoin", 1)
def stringjoin(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    arg = exec(arguments[0], stack)
    if arg.type != RuntimeValueType.Array:
        raise Exception(f"stringjoin: {arg} is not an array")
    return RuntimeValue.of("".join(x.to_string() for x in arg.value))


@builtin("sort", 1)
def sort(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    arg = exec(arguments[0], stack)
    if arg.type != RuntimeValueType.Array:
        raise Exception(f"sort: {arg} is not an array")
    return RuntimeValue.of(
        list(sorted(arg.value, key=cmp_to_key(RuntimeValue.compare)))
    )


@builtin("sortby", 2)
def sortby(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    target = exec(arguments[1], stack)
    if target.type != RuntimeValueType.Array:
        raise Exception(f"sortby: {target} is not an array")

    WithKey = List[Tuple[RuntimeValue, RuntimeValue]]
    with_key: WithKey = []
    for item in target.value:
        key = exec(arguments[0], add_runtime_value_to_stack(item, stack))
        with_key.append((key, item))

    def cmp(a: Tuple[RuntimeValue, RuntimeValue], b: Tuple[RuntimeValue, RuntimeValue]):
        return RuntimeValue.compare(a[0], b[0])

    post_sort = list(sorted(with_key, key=cmp_to_key(cmp)))
    return RuntimeValue.of([value for key, value in post_sort])


@builtin("<", 2)
def lt(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    return exec(arguments[0], stack) < exec(arguments[1], stack)


@builtin("<=", 2)
def lte(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    return exec(arguments[0], stack) <= exec(arguments[1], stack)


@builtin(">", 2)
def gt(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    return exec(arguments[0], stack) > exec(arguments[1], stack)


@builtin(">=", 2)
def gte(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    return exec(arguments[0], stack) >= exec(arguments[1], stack)


@builtin("values", 1)
def values(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    target = exec(arguments[0], stack)
    values = [target.access(key) for key in target.keys()]
    return RuntimeValue.of(values)


@builtin("groupby", 2)
def groupby(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    target = exec(arguments[1], stack)
    if target.type != RuntimeValueType.Array:
        raise Exception(f"groupby: {target} is not an array")
    mut = arguments[0]
    groups = {}
    for item in target.value:
        key = exec(mut, add_runtime_value_to_stack(item, stack)).to_string()
        if key not in groups:
            groups[key] = []
        groups[key].append(item)
    return RuntimeValue.of(groups)


@builtin("withindices", 1)
def withindices(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    target = exec(arguments[0], stack)
    if target.type != RuntimeValueType.Array:
        raise Exception(f"withindices: {target} is not an array")
    return RuntimeValue.of(list(enumerate(target.value)))


@builtin("entries", 1)
def entries(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    target = exec(arguments[0], stack)
    entries = [[key, target.access(key)] for key in target.keys()]
    return RuntimeValue.of(entries)


@builtin("fromentries", 1)
def fromentries(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    target = exec(arguments[0], stack)
    if target.type != RuntimeValueType.Array:
        raise Exception(f"fromentries: {target} is not an array")
    res = {}
    for entry in target.value:
        if entry.type != RuntimeValueType.Array:
            raise Exception(f"fromentries: {entry} is not an array")

        if len(entry.value) > 0:
            first = entry.value[0]
        else:
            first = RuntimeValue.of(None)

        if len(entry.value) > 1:
            second = entry.value[1]
        else:
            second = RuntimeValue.of(None)
        res[first.to_string()] = second
    return RuntimeValue.of(res)


@builtin("match", 2)
def match(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    pattern = exec(arguments[0], stack)
    target = exec(arguments[1], stack)
    if pattern.type == RuntimeValueType.Regex:
        return RuntimeValue.of(bool(pattern.value.search(target.value)))
    elif pattern.type == RuntimeValueType.String:
        compiled = re.compile(pattern.value)
        return RuntimeValue.of(bool(compiled.search(target.value)))
    else:
        raise Exception(f"match: {target} is not a string or regex")


@builtin("=~", 2)
def match_operator(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    return match(arguments[::-1], stack, exec)


@builtin("replace", 3)
def replace(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    pattern = exec(arguments[0], stack)
    replacement = exec(arguments[1], stack)
    target = exec(arguments[2], stack)
    if pattern.type == RuntimeValueType.Regex:
        if pattern.modifiers["global"]:
            res = pattern.value.sub(replacement.value, target.value)
        else:
            res = pattern.value.sub(replacement.value, target.value, 1)
        return RuntimeValue.of(res)
    elif pattern.type == RuntimeValueType.String:
        return RuntimeValue.of(
            target.value.replace(pattern.value, replacement.value, 1)
        )
    else:
        raise Exception(f"replace: {target} is not a string or regex")


@builtin("split", 2)
def split(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    delimiter = exec(arguments[0], stack)
    target = exec(arguments[1], stack)
    if target.type != RuntimeValueType.String:
        raise Exception(f"split: {target} is not a string")
    if delimiter.type == RuntimeValueType.String:
        separator = delimiter.value
        if separator == "":
            return RuntimeValue.of(list(target.value))
        return RuntimeValue.of(target.value.split(separator))
    elif delimiter.type == RuntimeValueType.Regex:
        return RuntimeValue.of(list(delimiter.value.split(target.value)))
    raise Exception(f"split: {delimiter} is not a string or regex")


@builtin("stringjoin", 2)
def stringjoin(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    delimiter = exec(arguments[0], stack)
    target = exec(arguments[1], stack)
    if target.type != RuntimeValueType.Array:
        raise Exception(f"stringjoin: {target} is not an array")
    if delimiter.type != RuntimeValueType.String:
        raise Exception(f"stringjoin: {delimiter} is not a string")
    arr = [entry.to_string() for entry in target.value]
    return RuntimeValue.of(delimiter.value.join(arr))


@builtin("summarize", 1)
def summarize(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    target = exec(arguments[0], stack)
    if target.type != RuntimeValueType.Array:
        raise Exception(f"summarize: {target} is not an array")
    for entry in target.value:
        if entry.type != RuntimeValueType.Number:
            raise Exception(f"summarize: inner value {entry} is not a number")
    arr = target.to_python()
    summary = {
        "max": max(arr),
        "min": min(arr),
        "mean": statistics.mean(arr),
        "median": statistics.median(arr),
        "variance": statistics.variance(arr),
        "stddev": statistics.stdev(arr),
    }
    return RuntimeValue.of(summary)


def _sequence_helper(arr: List[List[bool]], start=0) -> List[List[int]]:
    firstArray = arr[0]
    result: List[List[int]] = []
    for idx in range(start, len(firstArray)):
        if firstArray[idx]:
            if len(arr) == 1:
                result.append([idx])
            else:
                subResult = _sequence_helper(arr[1:], idx + 1)
                for i in range(len(subResult)):
                    result.append([idx] + subResult[i])
    return result


@builtin("sequence", 2, -1)
def sequence(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    predicates = arguments[:-1]
    target = exec(arguments[-1], stack)
    if target.type != RuntimeValueType.Array:
        raise Exception(f"sequence: {target} is not an array")
    bitmasks: List[List[bool]] = []
    for predicate in predicates:
        bitmask = []
        for i in range(len(target.value)):
            item = target.value[i]
            value = exec(predicate, add_runtime_value_to_stack(item, stack))
            bitmask.append(value.truthy())
        bitmasks.append(bitmask)
    indices_map = _sequence_helper(bitmasks)
    result: List[List[int]] = [
        [target.value[idx] for idx in indices] for indices in indices_map
    ]
    return RuntimeValue.of(result)
