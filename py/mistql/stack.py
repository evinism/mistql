from typing import Callable, List, Dict
from mistql.runtime_value import RuntimeValue, RuntimeValueType
from mistql.exceptions import MistQLReferenceError
from typeguard import typechecked

StackFrame = Dict[str, RuntimeValue]
Stack = List[StackFrame]


def add_runtime_value_to_stack(value: RuntimeValue, stack: Stack):
    new_stackframe = {"@": value}
    for key in value.keys():
        new_stackframe[key] = value.access(key)
    new_stack = stack.copy()
    new_stack.append(new_stackframe)
    return new_stack


def build_initial_stack(data: RuntimeValue, builtins: Dict[str, Callable]) -> Stack:
    dollar_var = {
        "@": data,
    }
    top_stack_entry = {
        "@": data,
    }
    if data.type == RuntimeValueType.Object:
        for key, value in data.value.items():
            top_stack_entry[key] = value
    for builtin in builtins:
        fn = RuntimeValue.create_function(builtins[builtin])
        dollar_var[builtin] = fn
    top_stack_entry["$"] = RuntimeValue.of(dollar_var)
    builtin_stackframe = {
        name: RuntimeValue.create_function(builtins[name]) for name in builtins
    }
    return [builtin_stackframe, top_stack_entry]


@typechecked
def find_in_stack(stack: Stack, name: str, absolute: bool) -> RuntimeValue:
    if absolute:
        stack = stack[:1]
    for frame in reversed(stack):
        if name in frame:
            return frame[name]
    raise MistQLReferenceError(f"Could not find {name} in stack")
