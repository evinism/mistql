from typing import Dict, Union, Callable, Optional, Any
from functools import lru_cache

from .execute import execute_outer
from .runtime_value import RuntimeValue
from .gardenwall import input_garden_wall, output_garden_wall
from .parse import parse


ExtrasDict = Dict[str, Union[RuntimeValue, Callable]]


class MistQLInstance:
    extras: ExtrasDict
    parse_lru_cache_size: int
    _cached_parse: Callable[[str], Any]

    def __init__(
        self, extras: Optional[ExtrasDict] = None,
        parse_lru_cache_size: int = 4,
        lazy: bool = True
    ):
        self.extras = extras or {}
        self.parse_lru_cache_size = parse_lru_cache_size
        self.lazy = lazy
        self._cached_parse = lru_cache(maxsize=parse_lru_cache_size)(parse)

    def query(self, query: str, data: Any):
        ast = self._cached_parse(query)
        data = input_garden_wall(data, self.lazy)
        result = execute_outer(ast, data, self.extras)
        return_value = output_garden_wall(result)
        return return_value


default_instance = MistQLInstance()
