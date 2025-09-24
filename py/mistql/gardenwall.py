from typing import Any
from mistql.runtime_value import RuntimeValue


def input_garden_wall(data: Any, lazy: bool = False) -> RuntimeValue:
    return RuntimeValue.of(data, lazy)


def output_garden_wall(data: RuntimeValue) -> Any:
    return data.to_python()
