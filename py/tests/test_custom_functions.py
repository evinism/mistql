import pytest
from mistql import MistQLInstance, RuntimeValue


def test_basic_custom_function():
    def add_one(x):
        return x + 1
    mq = MistQLInstance({
        "add_one": add_one
    })
    assert mq.query("add_one 1", None) == 2


def test_variadic_custom_function():
    def add(*args):
        return sum(args)
    mq = MistQLInstance({
        "add": add
    })
    assert mq.query("add 1 2 3", None) == 6
    assert mq.query("add 1 2 3 4", None) == 10


def test_variadic_custom_function_with_positionals():
    def add(x, *args):
        return x + sum(args)
    mq = MistQLInstance({
        "add": add
    })
    assert mq.query("add 5", None) == 5
    assert mq.query("add 5 2", None) == 7
    assert mq.query("add 5 2 3", None) == 10


def test_bad_arity_raises():
    def add(x, y):
        return x + y
    mq = MistQLInstance({
        "add": add
    })
    assert mq.query("add 1 2", None) == 3
    with pytest.raises(Exception):
        mq.query("add 1", None)


def test_kw_only_args_throw():
    def add(*, x):
        return x
    with pytest.raises(Exception):
        # Should move to erroring on instance creation
        mq = MistQLInstance({
            "add": add
        })
        mq.query("add 1", None)


def test_using_runtime_value_construction():
    def add(args, stack, exec):
        return RuntimeValue.of(exec(args[0], stack).value + exec(args[1], stack).value)
    mq = MistQLInstance({
        "add": RuntimeValue.wrap_function_def(add)
    })
    assert mq.query("add 1 2", None) == 3
