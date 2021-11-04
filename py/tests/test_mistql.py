from mistql import __version__, query


def test_version():
    assert __version__ == "0.0.0"


def test_query_is_callable():
    assert query and callable(query)
