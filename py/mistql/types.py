# This file contains lots of stuff that doesn't yet work.
# But that i should get to work at some point

RuntimeFuncDef = Callable[
    [
        List[Expression],
        Stack,
        Callable[[Expression, Stack], RuntimeValue],
    ],
    RuntimeValue,
]
