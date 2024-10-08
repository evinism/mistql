from typing import Dict, Union, Callable, Optional, Any

from .execute import execute_outer
from .runtime_value import RuntimeValue
from .gardenwall import input_garden_wall, output_garden_wall
from .parse import parse


ExtrasDict = Dict[str, Union[RuntimeValue, Callable]]


class MistQLInstance:
    extras: ExtrasDict

    def __init__(self, extras: Optional[ExtrasDict] = None):
        self.extras = extras or {}

    def query(self, query: str, data: Any):
        ast = parse(query)
        data = input_garden_wall(data)
        result = execute_outer(ast, data, self.extras)
        return_value = output_garden_wall(result)
        return return_value


default_instance = MistQLInstance()
