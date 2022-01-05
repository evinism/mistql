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


def reverse(arguments: Args, stack: Stack, execute: Exec) -> RuntimeValue:
    if len(arguments) != 1:
        raise Exception("reverse takes one argument")
    arg = execute(arguments[0], stack)
    if arg.type != RuntimeValueType.Array:
        raise Exception(f"reverse takes an array, got {arg}")
    import pdb

    pdb.set_trace()
    return RuntimeValue.of(list(reversed(arg.value)))


builtins = {
    "reverse": reverse,
}
