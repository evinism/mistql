from typing import List, Mapping, Callable, Union, Dict
from mistql.runtime_value import RuntimeValue
from mistql.exceptions import MistQLReferenceError
from typeguard import typechecked

StackFrame = Dict[str, RuntimeValue]
Stack = List[StackFrame]


def make_stack_entry_from_runtime_value(value: RuntimeValue) -> StackFrame:
    new_stackframe = {"@": value}
    for key in value.keys():
        new_stackframe[key] = value.access(key)
    return new_stackframe


def add_runtime_value_to_stack(value: RuntimeValue, stack: Stack):
    new_stackframe = make_stack_entry_from_runtime_value(value)
    new_stack = stack.copy()
    new_stack.append(new_stackframe)
    return new_stack


def build_initial_stack(
    data: RuntimeValue,
    builtins: Mapping[str, Callable],
    extras: Mapping[str, Union[Callable, RuntimeValue]]
) -> Stack:
    functions_frame: StackFrame = {}
    for key, builtin in builtins.items():
        functions_frame[key] = RuntimeValue.wrap_function_def(builtin)
    for key, value in extras.items():
        if isinstance(value, RuntimeValue):
            functions_frame[key] = value
        else:
            functions_frame[key] = RuntimeValue.from_py_func(value)

    dollar_var_dict = {"@": data}
    dollar_var_dict.update(functions_frame)

    return [
        functions_frame,
        {"$": RuntimeValue.of(dollar_var_dict)},
        make_stack_entry_from_runtime_value(data)
    ]


@typechecked
def find_in_stack(stack: Stack, name: str, absolute: bool) -> RuntimeValue:
    if absolute:
        stack = stack[:1]
    for frame in reversed(stack):
        if name in frame:
            return frame[name]
    raise MistQLReferenceError(f"Could not find {name} in stack")
