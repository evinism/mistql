import re
import statistics
from functools import cmp_to_key
from typing import Callable, Dict, List, Tuple, Union

from mistql.exceptions import (MistQLRuntimeError, MistQLTypeError,
                               OpenAnIssueIfYouGetThisError)
from mistql.expression import BaseExpression, RefExpression
from mistql.runtime_value import RuntimeValue, RuntimeValueType, assert_type
from mistql.stack import Stack, add_runtime_value_to_stack

Args = List[BaseExpression]
Exec = Callable[[BaseExpression, Stack], RuntimeValue]

FunctionDefinitionType = Callable[
    [Args, Stack, Exec],
    RuntimeValue,
]

builtins: Dict[str, FunctionDefinitionType] = {}

RVT = RuntimeValueType


def builtin(name: str, min_args: int, max_args: Union[None, int] = None):
    if max_args is None:
        max_args = min_args

    def builtin_decorator(fn: FunctionDefinitionType) -> FunctionDefinitionType:
        def wrapped(arguments: Args, stack: Stack, exec: Exec):
            if not min_args < 0 and len(arguments) < min_args:
                raise MistQLRuntimeError(f"{name} takes at least {min_args} arguments")
            if (max_args is not None) and (
                not max_args < 0 and len(arguments) > max_args
            ):
                raise MistQLRuntimeError(f"{name} takes at most {max_args} arguments")
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
    arg = assert_type(exec(arguments[0], stack), RVT.Array)
    return RuntimeValue.of(list(reversed(arg.value)))


@builtin("-/unary", 1)
def unary_minus(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    res = assert_type(exec(arguments[0], stack), RVT.Number)
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
        raise MistQLTypeError(f"add: {left} and {right} are not the same type")
    if left.type in {
        RVT.Number,
        RVT.String,
        RVT.Array,
    }:
        return RuntimeValue.of(left.value + right.value)
    raise MistQLTypeError(f"add: {left.type} is not supported")


@builtin("-", 2)
def subtract(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    left = assert_type(exec(arguments[0], stack), RVT.Number)
    right = assert_type(exec(arguments[1], stack), RVT.Number)
    return RuntimeValue.of(left.value - right.value)


@builtin("*", 2)
def multiply(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    left = assert_type(exec(arguments[0], stack), RVT.Number)
    right = assert_type(exec(arguments[1], stack), RVT.Number)
    return RuntimeValue.of(left.value * right.value)


@builtin("/", 2)
def divide(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    left = assert_type(exec(arguments[0], stack), RVT.Number)
    right = assert_type(exec(arguments[1], stack), RVT.Number)
    return RuntimeValue.of(left.value / right.value)


@builtin("%", 2)
def mod(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    left = assert_type(exec(arguments[0], stack), RVT.Number)
    right = assert_type(exec(arguments[1], stack), RVT.Number)
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
    arg = assert_type(exec(arguments[0], stack), RVT.Array)
    return RuntimeValue.of(len(arg.value))


@builtin("keys", 1)
def keys(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    return RuntimeValue.of(exec(arguments[0], stack).keys())


@builtin(".", 2)
def dot(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    left = exec(arguments[0], stack)
    right = arguments[1]
    if not isinstance(right, RefExpression):
        raise MistQLRuntimeError("dot: RHS of the dot operator is not a ref")
    return _index_single(RuntimeValue.of(right.name), left)


@builtin("map", 2)
def map(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    mutation = arguments[0]
    operand = assert_type(exec(arguments[1], stack), RVT.Array)
    out: List[RuntimeValue] = []
    for item in operand.value:
        res = exec(mutation, add_runtime_value_to_stack(item, stack))
        out.append(res)
    return RuntimeValue.of(out)


@builtin("reduce", 3)
def reduce(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    mutation = arguments[0]
    initial = exec(arguments[1], stack)
    operand = assert_type(exec(arguments[2], stack), RVT.Array)
    out = initial
    for item in operand.value:
        acc_cur = RuntimeValue.of([out, item])
        out = exec(mutation, add_runtime_value_to_stack(acc_cur, stack))
    return out


@builtin("filter", 2)
def filter(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    mutation = arguments[0]
    operand = assert_type(exec(arguments[1], stack), RVT.Array)
    out: List[RuntimeValue] = []
    for item in operand.value:
        res = exec(mutation, add_runtime_value_to_stack(item, stack))
        if res:
            out.append(item)
    return RuntimeValue.of(out)


@builtin("mapvalues", 2)
def mapvalues(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    mutation = arguments[0]
    operand = assert_type(exec(arguments[1], stack), RVT.Object)
    out: Dict[str, RuntimeValue] = {}
    for key, value in operand.value.items():
        res = exec(mutation, add_runtime_value_to_stack(value, stack))
        out[key] = res
    return RuntimeValue.of(out)


@builtin("mapkeys", 2)
def mapkeys(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    mutation = arguments[0]
    operand = assert_type(exec(arguments[1], stack), RVT.Object)
    out: Dict[str, RuntimeValue] = {}
    for key, value in operand.value.items():
        res = exec(mutation, add_runtime_value_to_stack(RuntimeValue.of(key), stack))
        out[res.to_string()] = value
    return RuntimeValue.of(out)


@builtin("filtervalues", 2)
def filtervalues(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    mutation = arguments[0]
    operand = assert_type(exec(arguments[1], stack), RVT.Object)
    out: Dict[str, RuntimeValue] = {}
    for key, value in operand.value.items():
        res = exec(mutation, add_runtime_value_to_stack(value, stack))
        if res:
            out[key] = value
    return RuntimeValue.of(out)


@builtin("filterkeys", 2)
def filterkeys(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    mutation = arguments[0]
    operand = assert_type(exec(arguments[1], stack), RVT.Object)
    out: Dict[str, RuntimeValue] = {}
    for key, value in operand.value.items():
        res = exec(mutation, add_runtime_value_to_stack(RuntimeValue.of(key), stack))
        if res:
            out[key] = value
    return RuntimeValue.of(out)


@builtin("find", 2)
def find(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    mutation = arguments[0]
    operand = assert_type(exec(arguments[1], stack), RVT.Array)
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
    assert_type(operand, {RVT.Array, RVT.String})

    if index_one.type == RVT.Null:
        index_one = RuntimeValue.of(0)
    if index_two.type == RVT.Null:
        index_two = RuntimeValue.of(len(operand.value))
    if index_one.type != RVT.Number or index_two.type != RVT.Number:
        raise MistQLRuntimeError("index: Non-numbers cannot be used on arrays")
    index_one_num = index_one.value
    index_two_num = index_two.value
    if index_one_num % 1 != 0 or index_two_num % 1 != 0:
        raise MistQLRuntimeError("index: Non-integers cannot be used on arrays")
    if index_one_num < 0:
        index_one_num = len(operand.value) + index_one_num
    if index_two_num < 0:
        index_two_num = len(operand.value) + index_two_num
    return RuntimeValue.of(operand.value[int(index_one_num) : int(index_two_num)])


def _index_single(index: RuntimeValue, operand: RuntimeValue):
    if operand.type == RVT.Array or operand.type == RVT.String:
        assert_type(index, RVT.Number)
        index_num = index.value
        if index_num % 1 != 0:
            raise MistQLRuntimeError("index: Non-integers cannot be used on arrays")
        if index_num < 0:
            index_num = len(operand.value) + index_num
        if index_num < 0 or index_num >= len(operand.value):
            return RuntimeValue.of(None)
        return RuntimeValue.of(operand.value[int(index_num)])
    elif operand.type == RVT.Object:
        return operand.access(assert_type(index, RVT.String).value)
    elif operand.type == RVT.Null:
        assert_type(index, {RVT.Number, RVT.String})
        return RuntimeValue.of(None)
    else:
        raise MistQLRuntimeError(f"index: Cannot index {operand.type}")


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
    pattern = assert_type(exec(arguments[0], stack), RVT.String)

    if len(arguments) == 2:
        flags = assert_type(exec(arguments[1], stack), RVT.String)
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
        RVT.Regex,
        re.compile(pattern.value, flags=flags_int),
        modifiers={"global": is_global},
    )


@builtin("sort", 1)
def sort(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    arg = assert_type(exec(arguments[0], stack), RVT.Array)
    for entry in arg.value:
        if not entry.comparable():
            raise MistQLRuntimeError("sort: Cannot sort non-comparable values")
    return RuntimeValue.of(
        list(sorted(arg.value, key=cmp_to_key(RuntimeValue.compare)))
    )


@builtin("sortby", 2)
def sortby(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    target = assert_type(exec(arguments[1], stack), RVT.Array)
    WithKey = List[Tuple[RuntimeValue, RuntimeValue]]
    with_key: WithKey = []
    for item in target.value:
        key = exec(arguments[0], add_runtime_value_to_stack(item, stack))
        if not key.comparable():
            raise MistQLRuntimeError("sort: Cannot sort non-comparable values")
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
    target = assert_type(exec(arguments[1], stack), RVT.Array)
    mut = arguments[0]
    groups: Dict[str, List[RuntimeValue]] = {}
    for item in target.value:
        key = exec(mut, add_runtime_value_to_stack(item, stack)).to_string()
        if key not in groups:
            groups[key] = []
        groups[key].append(item)
    return RuntimeValue.of(groups)


@builtin("withindices", 1)
def withindices(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    target = assert_type(exec(arguments[0], stack), RVT.Array)
    return RuntimeValue.of(list(enumerate(target.value)))


@builtin("entries", 1)
def entries(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    target = exec(arguments[0], stack)
    entries = [[key, target.access(key)] for key in target.keys()]
    return RuntimeValue.of(entries)


@builtin("fromentries", 1)
def fromentries(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    target = assert_type(exec(arguments[0], stack), RVT.Array)
    res = {}
    for entry in target.value:
        assert_type(entry, RVT.Array)
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
    assert_type(pattern, {RVT.String, RVT.Regex})
    if pattern.type == RVT.Regex:
        return RuntimeValue.of(bool(pattern.value.search(target.value)))
    elif pattern.type == RVT.String:
        compiled = re.compile(pattern.value)
        return RuntimeValue.of(bool(compiled.search(target.value)))
    raise OpenAnIssueIfYouGetThisError(
        "Unexpectedly reaching end of function in match call."
    )


@builtin("=~", 2)
def match_operator(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    return match(arguments[::-1], stack, exec)


@builtin("replace", 3)
def replace(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    pattern = exec(arguments[0], stack)
    replacement = exec(arguments[1], stack)
    target = exec(arguments[2], stack)
    assert_type(pattern, {RVT.String, RVT.Regex})
    if pattern.type == RVT.Regex:
        if pattern.modifiers["global"]:
            res = pattern.value.sub(replacement.value, target.value)
        else:
            res = pattern.value.sub(replacement.value, target.value, 1)
        return RuntimeValue.of(res)
    elif pattern.type == RVT.String:
        return RuntimeValue.of(
            target.value.replace(pattern.value, replacement.value, 1)
        )
    raise OpenAnIssueIfYouGetThisError(
        "Unexpectedly reaching end of function in match call."
    )


@builtin("split", 2)
def split(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    delimiter = assert_type(exec(arguments[0], stack), {RVT.String, RVT.Regex})
    target = assert_type(exec(arguments[1], stack), RVT.String)
    if delimiter.type == RVT.String:
        separator = delimiter.value
        if separator == "":
            return RuntimeValue.of(list(target.value))
        return RuntimeValue.of(target.value.split(separator))
    elif delimiter.type == RVT.Regex:
        return RuntimeValue.of(list(delimiter.value.split(target.value)))
    raise OpenAnIssueIfYouGetThisError(
        "Unexpectedly reaching end of function in match call."
    )


@builtin("stringjoin", 2)
def stringjoin(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    delimiter = assert_type(exec(arguments[0], stack), RVT.String)
    target = assert_type(exec(arguments[1], stack), RVT.Array)
    arr = [entry.to_string() for entry in target.value]
    return RuntimeValue.of(delimiter.value.join(arr))


@builtin("sum", 1)
def sum(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    target = assert_type(exec(arguments[0], stack), RVT.Array)
    total = 0
    for entry in target.value:
        total += assert_type(entry, RVT.Number).value
    return RuntimeValue.of(total)


@builtin("summarize", 1)
def summarize(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    target = assert_type(exec(arguments[0], stack), RVT.Array)
    for entry in target.value:
        assert_type(entry, RVT.Number)
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
    target = assert_type(exec(arguments[-1], stack), RVT.Array)
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


@builtin("flatten", 1)
def flatten(arguments: Args, stack: Stack, exec: Exec) -> RuntimeValue:
    target = assert_type(exec(arguments[0], stack), RVT.Array)
    result: List[RuntimeValue] = []
    for entry in target.value:
        result.extend(assert_type(entry, RVT.Array).value)
    return RuntimeValue.of(result)
