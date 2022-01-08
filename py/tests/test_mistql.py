from mistql import __version__, query
import toml
import json
import os


def test_version():
    pyproject_path = os.path.join(os.path.dirname(__file__), "..", "pyproject.toml")
    meta_file_path = os.path.join(os.path.dirname(__file__), "..", "..", "meta.json")
    assert __version__ == toml.load(pyproject_path)["tool"]["poetry"]["version"]
    with open(meta_file_path) as f:
        assert __version__ == json.load(f)["version"]


def test_query_is_callable():
    assert query and callable(query)
