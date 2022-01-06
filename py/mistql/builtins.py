from typing import List, Dict, Callable, Tuple
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
    return RuntimeValue.of(not res)


def if_else(arguments: Args, stack: Stack, execute: Exec) -> RuntimeValue:
    if len(arguments) != 3:
        raise Exception("if takes three arguments")
    condition = execute(arguments[0], stack)
    if condition:
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
    return left == right


def neq(arguments: Args, stack: Stack, execute: Exec) -> RuntimeValue:
    if len(arguments) != 2:
        raise Exception("neq takes two arguments")
    left = execute(arguments[0], stack)
    right = execute(arguments[1], stack)
    return left != right


def and_fn(arguments: Args, stack: Stack, execute: Exec) -> RuntimeValue:
    if len(arguments) != 2:
        raise Exception("and takes two arguments")
    left = execute(arguments[0], stack)
    right = execute(arguments[1], stack)
    if left:
        return right
    else:
        return left


def or_fn(arguments: Args, stack: Stack, execute: Exec) -> RuntimeValue:
    if len(arguments) != 2:
        raise Exception("or takes two arguments")
    left = execute(arguments[0], stack)
    right = execute(arguments[1], stack)
    if left:
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


def reduce(arguments: Args, stack: Stack, execute: Exec) -> RuntimeValue:
    if len(arguments) != 3:
        raise Exception("reduce takes three arguments")
    initial = execute(arguments[1], stack)
    mutation = arguments[0]
    operand = execute(arguments[2], stack)
    if operand.type != RuntimeValueType.Array:
        raise Exception(f"reduce: {operand} is not an array")
    out = initial
    for item in operand.value:
        acc_cur = RuntimeValue.of([out, item])
        out = execute(mutation, add_runtime_value_to_stack(acc_cur, stack))
    return out


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
        if res:
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


def filtervalues(arguments: Args, stack: Stack, execute: Exec) -> RuntimeValue:
    if len(arguments) != 2:
        raise Exception("filtervalues takes two arguments")
    mutation = arguments[0]
    operand = execute(arguments[1], stack)
    if operand.type != RuntimeValueType.Object:
        raise Exception(f"filtervalues: {operand} is not an object")
    out: Dict[str, RuntimeValue] = {}
    for key, value in operand.value.items():
        res = execute(mutation, add_runtime_value_to_stack(value, stack))
        if res:
            out[key] = value
    return RuntimeValue.of(out)


def filterkeys(arguments: Args, stack: Stack, execute: Exec) -> RuntimeValue:
    if len(arguments) != 2:
        raise Exception("filterkeys takes two arguments")
    mutation = arguments[0]
    operand = execute(arguments[1], stack)
    if operand.type != RuntimeValueType.Object:
        raise Exception(f"filterkeys: {operand} is not an object")
    out: Dict[str, RuntimeValue] = {}
    for key, value in operand.value.items():
        res = execute(mutation, add_runtime_value_to_stack(RuntimeValue.of(key), stack))
        if res:
            out[key] = value
    return RuntimeValue.of(out)


def find(arguments: Args, stack: Stack, execute: Exec) -> RuntimeValue:
    if len(arguments) != 2:
        raise Exception("find takes two arguments")
    mutation = arguments[0]
    operand = execute(arguments[1], stack)
    if operand.type != RuntimeValueType.Array:
        raise Exception(f"find: {operand} is not an array")
    for item in operand.value:
        res = execute(mutation, add_runtime_value_to_stack(item, stack))
        if res:
            return item
    return RuntimeValue.of(None)


def apply(arguments: Args, stack: Stack, execute: Exec) -> RuntimeValue:
    if len(arguments) != 2:
        raise Exception("apply takes two arguments")
    target = execute(arguments[1], stack)
    return execute(arguments[0], add_runtime_value_to_stack(target, stack))


def _index_double(
    operand: RuntimeValue,
    index_one: RuntimeValue,
    index_two: RuntimeValue,
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
        return RuntimeValue.of(operand.value[int(index_num)])
    else:
        return operand.access(index.to_string())


def index(arguments: Args, stack: Stack, execute: Exec) -> RuntimeValue:
    if len(arguments) not in {2, 3}:
        raise Exception("index takes two to three arguments")
    if len(arguments) == 3:
        return _index_double(
            execute(arguments[2], stack),
            execute(arguments[0], stack),
            execute(arguments[1], stack),
        )
    else:
        return _index_single(execute(arguments[1], stack), execute(arguments[0], stack))


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
        flags = execute(arguments[1], stack)
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


def stringjoin(arguments: Args, stack: Stack, execute: Exec) -> RuntimeValue:
    if len(arguments) != 1:
        raise Exception("stringjoin takes one argument")
    arg = execute(arguments[0], stack)
    if arg.type != RuntimeValueType.Array:
        raise Exception(f"stringjoin: {arg} is not an array")
    return RuntimeValue.of("".join(x.to_string() for x in arg.value))


def sort(arguments: Args, stack: Stack, execute: Exec) -> RuntimeValue:
    if len(arguments) != 1:
        raise Exception("sort takes one argument")
    arg = execute(arguments[0], stack)
    if arg.type != RuntimeValueType.Array:
        raise Exception(f"sort: {arg} is not an array")
    return RuntimeValue.of(
        list(sorted(arg.value, key=cmp_to_key(RuntimeValue.compare)))
    )


def sortby(arguments: Args, stack: Stack, execute: Exec) -> RuntimeValue:
    if len(arguments) != 2:
        raise Exception("sortby takes two arguments")
    target = execute(arguments[1], stack)
    if target.type != RuntimeValueType.Array:
        raise Exception(f"sortby: {target} is not an array")

    WithKey = List[Tuple[RuntimeValue, RuntimeValue]]
    with_key: WithKey = []
    for item in target.value:
        key = execute(arguments[0], add_runtime_value_to_stack(item, stack))
        with_key.append((key, item))

    def cmp(a: Tuple[RuntimeValue, RuntimeValue], b: Tuple[RuntimeValue, RuntimeValue]):
        return RuntimeValue.compare(a[0], b[0])

    post_sort = list(sorted(with_key, key=cmp_to_key(cmp)))
    return RuntimeValue.of([value for key, value in post_sort])


def lt(arguments: Args, stack: Stack, execute: Exec) -> RuntimeValue:
    if len(arguments) != 2:
        raise Exception("lt takes two arguments")
    return execute(arguments[0], stack) < execute(arguments[1], stack)


def lte(arguments: Args, stack: Stack, execute: Exec) -> RuntimeValue:
    if len(arguments) != 2:
        raise Exception("lte takes two arguments")
    return execute(arguments[0], stack) <= execute(arguments[1], stack)


def gt(arguments: Args, stack: Stack, execute: Exec) -> RuntimeValue:
    if len(arguments) != 2:
        raise Exception("gt takes two arguments")
    return execute(arguments[0], stack) > execute(arguments[1], stack)


def gte(arguments: Args, stack: Stack, execute: Exec) -> RuntimeValue:
    if len(arguments) != 2:
        raise Exception("gte takes two arguments")
    return execute(arguments[0], stack) >= execute(arguments[1], stack)


def values(arguments: Args, stack: Stack, execute: Exec) -> RuntimeValue:
    if len(arguments) != 1:
        raise Exception("values takes one argument")
    target = execute(arguments[0], stack)
    keys = target.keys()
    values = [target.access(key) for key in keys]
    return RuntimeValue.of(values)


def groupby(arguments: Args, stack: Stack, execute: Exec) -> RuntimeValue:
    if len(arguments) != 2:
        raise Exception("groupby takes two arguments")
    target = execute(arguments[1], stack)
    if target.type != RuntimeValueType.Array:
        raise Exception(f"groupby: {target} is not an array")
    mut = arguments[0]
    groups = {}
    for item in target.value:
        key = execute(mut, add_runtime_value_to_stack(item, stack)).to_string()
        if key not in groups:
            groups[key] = []
        groups[key].append(item)
    return RuntimeValue.of(groups)


def withindices(arguments: Args, stack: Stack, execute: Exec) -> RuntimeValue:
    if len(arguments) != 1:
        raise Exception("withindices takes one argument")
    target = execute(arguments[0], stack)
    if target.type != RuntimeValueType.Array:
        raise Exception(f"withindices: {target} is not an array")
    return RuntimeValue.of(list(enumerate(target.value)))


def entries(arguments: Args, stack: Stack, execute: Exec) -> RuntimeValue:
    if len(arguments) != 1:
        raise Exception("entries takes one argument")
    target = execute(arguments[0], stack)
    return RuntimeValue.of([[key, target.access(key)] for key in target.keys()])


def fromentries(arguments: Args, stack: Stack, execute: Exec) -> RuntimeValue:
    if len(arguments) != 1:
        raise Exception("fromentries takes one argument")
    target = execute(arguments[0], stack)
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


def match(arguments: Args, stack: Stack, execute: Exec) -> RuntimeValue:
    if len(arguments) != 2:
        raise Exception("match takes two arguments")
    target = execute(arguments[1], stack)
    pattern = execute(arguments[0], stack)
    if pattern.type == RuntimeValueType.Regex:
        return RuntimeValue.of(bool(pattern.value.match(target.value)))
    elif pattern.type == RuntimeValueType.String:
        return RuntimeValue.of(target == pattern)
    else:
        raise Exception(f"match: {target} is not a string or regex")


def match_operator(arguments: Args, stack: Stack, execute: Exec) -> RuntimeValue:
    if len(arguments) != 2:
        raise Exception("match takes two arguments")
    reversed_args = arguments[::-1]
    return match(reversed_args, stack, execute)


def replace(arguments: Args, stack: Stack, execute: Exec) -> RuntimeValue:
    if len(arguments) != 3:
        raise Exception("replace takes three arguments")
    target = execute(arguments[2], stack)
    pattern = execute(arguments[0], stack)
    replacement = execute(arguments[1], stack)
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


def split(arguments: Args, stack: Stack, execute: Exec) -> RuntimeValue:
    if len(arguments) != 2:
        raise Exception("split takes two arguments")
    target = execute(arguments[1], stack)
    if target.type != RuntimeValueType.String:
        raise Exception(f"split: {target} is not a string")
    delimiter = execute(arguments[0], stack)
    if delimiter.type == RuntimeValueType.String:
        separator = delimiter.value
        if separator == "":
            return RuntimeValue.of(list(target.value))
        return RuntimeValue.of(target.value.split(separator))
    elif delimiter.type == RuntimeValueType.Regex:
        return RuntimeValue.of(list(delimiter.value.split(target.value)))
    raise Exception(f"split: {delimiter} is not a string or regex")


def stringjoin(arguments: Args, stack: Stack, execute: Exec) -> RuntimeValue:
    if len(arguments) != 2:
        raise Exception("stringjoin takes two arguments")
    target = execute(arguments[1], stack)
    if target.type != RuntimeValueType.Array:
        raise Exception(f"stringjoin: {target} is not an array")
    delimiter = execute(arguments[0], stack)
    if delimiter.type != RuntimeValueType.String:
        raise Exception(f"stringjoin: {delimiter} is not a string")
    arr = [entry.to_string() for entry in target.value]
    return RuntimeValue.of(delimiter.value.join(arr))


def summarize(arguments: Args, stack: Stack, execute: Exec) -> RuntimeValue:
    if len(arguments) != 1:
        raise Exception("summarize takes one argument")
    target = execute(arguments[0], stack)
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


def sequence(arguments: Args, stack: Stack, execute: Exec) -> RuntimeValue:
    if len(arguments) < 2:
        raise Exception("sequence takes at least two arguments")
    target = execute(arguments[-1], stack)
    predicates = arguments[:-1]
    if target.type != RuntimeValueType.Array:
        raise Exception(f"sequence: {target} is not an array")
    bitmasks: List[List[bool]] = []
    for predicate in predicates:
        bitmask = []
        for i in range(len(target.value)):
            item = target.value[i]
            value = execute(predicate, add_runtime_value_to_stack(item, stack))
            bitmask.append(value.truthy())
        bitmasks.append(bitmask)
    indices_map = _sequence_helper(bitmasks)
    result: List[List[int]] = [
        [target.value[idx] for idx in indices] for indices in indices_map
    ]
    return RuntimeValue.of(result)


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
    ">": gt,
    ">=": gte,
    "<": lt,
    "<=": lte,
    "&&": and_fn,
    "||": or_fn,
    "=~": match_operator,
    "apply": apply,
    "count": count,
    "keys": keys,
    "entries": entries,
    "fromentries": fromentries,
    "match": match,
    "split": split,
    "values": values,
    "groupby": groupby,
    "float": float,
    "map": map,
    "filter": filter,
    "reduce": reduce,
    "find": find,
    "index": index,
    "string": string,
    "sort": sort,
    "sortby": sortby,
    "mapvalues": mapvalues,
    "mapkeys": mapkeys,
    "filtervalues": filtervalues,
    "filterkeys": filterkeys,
    "if": if_else,
    "reverse": reverse,
    "regex": regex,
    "log": log,
    "stringjoin": stringjoin,
    "withindices": withindices,
    "replace": replace,
    "summarize": summarize,
    "sequence": sequence,
}
