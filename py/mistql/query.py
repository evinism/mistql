from typing import Any

from .instance import default_instance

def query(query: str, data: Any) -> Any:
    """
    Executes a query on a given data.

    :param query: The query to execute.
    :param data: The data to query.
    :return: The result of the query.
    """
    return default_instance.query(query, data)