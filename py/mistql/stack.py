from typing import Callable, List, Dict
from mistql.runtime_value import RuntimeValue, RuntimeValueType

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
        top_stack_entry[builtin] = fn
        dollar_var[builtin] = fn
    top_stack_entry["$"] = RuntimeValue.of(dollar_var)

    return [top_stack_entry]
