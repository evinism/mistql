from mistql import __version__, query
import toml
import os


def test_version():
    pyproject_path = os.path.join(os.path.dirname(__file__), "..", "pyproject.toml")
    assert __version__ == toml.load(pyproject_path)["tool"]["poetry"]["version"]


def test_query_is_callable():
    assert query and callable(query)
