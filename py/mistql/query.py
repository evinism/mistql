from typing import Any

from .execute import execute_outer
from .gardenwall import input_garden_wall, output_garden_wall
from .parse import parse


def query(query: str, raw_data: Any) -> Any:
    """
    Executes a query on a given data.

    :param query: The query to execute.
    :param data: The data to query.
    :return: The result of the query.
    """
    ast = parse(query)
    data = input_garden_wall(raw_data)
    return output_garden_wall(execute_outer(ast, data))
