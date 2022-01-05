from typing import List, Dict
from mistql.runtime_value import RuntimeValue

StackFrame = Dict[str, RuntimeValue]
Stack = List[StackFrame]
