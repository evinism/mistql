from typing import List, Dict
from mistql.runtime_value import RuntimeValue

StackFrame = Dict[str, RuntimeValue]
Stack = List[StackFrame]


def add_runtime_value_to_stack(value: RuntimeValue, stack: Stack):
    new_stackframe = {
        "@": value
    }
    for key in value.keys():
        new_stackframe[key] = value.access(key)
    new_stack = stack.copy()
    new_stack.append(new_stackframe)
    return new_stack