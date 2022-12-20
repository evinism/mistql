from typing import Any, Dict, Union, Callable

from .execute import execute_outer
from .runtime_value import RuntimeValue
from .gardenwall import input_garden_wall, output_garden_wall
from .parse import parse


class MistQLInstance:
    def __init__(self, extras: Dict[str, Union[RuntimeValue, Callable]]=None):
        self.extras = extras or {}
    
    def query(self, query, data):
        ast = parse(query)
        data = input_garden_wall(data)
        result = execute_outer(ast, data, self.extras)
        return_value = output_garden_wall(result)
        return return_value

default_instance = MistQLInstance()
