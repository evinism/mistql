from mistql import RuntimeValue
from mistql.runtime_value import LazyRuntimeValue


def test_basic_lazy_creation():
    value = RuntimeValue.of([1, 2, 3], lazy=True)
    assert value.to_python() == [1, 2, 3]
    value = RuntimeValue.of({"a": 1, "b": 2, "c": 3}, lazy=True)
    assert value.to_python() == {"a": 1, "b": 2, "c": 3}


# Stability tests aren't guarantees, but they do validate caching is working
def test_value_stability_for_single_index_arrays():
    value = RuntimeValue.of([{"a": 1}, {"a": 2}, {"a": 3}], lazy=True)
    assert value.index(0) is value.index(0)
    assert value.index(1) is value.index(1)
    assert value.index(2) is value.index(2)


def test_value_stability_for_single_index_arrays():
    value = RuntimeValue.of([{"a": 1}, {"a": 2}, {"a": 3}], lazy=True)
    assert value.index(0) is value.index(0)
    assert value.index(1) is value.index(1)
    assert value.index(2) is value.index(2)


def test_value_stability_for_objects():
    value = RuntimeValue.of({"a": {"b": 1}, "b": {"b": 2}, "c": {"b": 3}}, lazy=True)
    assert value.access("a") is value.access("a")
    assert value.access("b") is value.access("b")
    assert value.access("c") is value.access("c")


def test_lazy_equality_for_arrays():
    value = RuntimeValue.of([{"a": 1}, {"a": 2}, {"a": 3}], lazy=True)
    assert value == value
    assert value == RuntimeValue.of([{"a": 1}, {"a": 2}, {"a": 3}], lazy=True)
    assert value == RuntimeValue.of([{"a": 1}, {"a": 2}, {"a": 3}], lazy=False)


def test_lazy_equality_for_objects():
    value = RuntimeValue.of({"a": {"b": 1}, "b": {"b": 2}, "c": {"b": 3}}, lazy=True)
    assert value == value
    assert value == RuntimeValue.of(
        {"a": {"b": 1}, "b": {"b": 2}, "c": {"b": 3}}, lazy=True
    )
    assert value == RuntimeValue.of(
        {"a": {"b": 1}, "b": {"b": 2}, "c": {"b": 3}}, lazy=False
    )


def test_laziness_with_array_indexing(monkeypatch):
    original_of = RuntimeValue.of

    def of_mock(*args, **kwargs):
        of_mock.called += 1
        return original_of(*args, **kwargs)

    of_mock.called = 0
    monkeypatch.setattr(RuntimeValue, "of", staticmethod(of_mock))

    value = RuntimeValue.of([{"a": 1}, {"a": 2}, {"a": 3}], lazy=True)
    # Once to create the value, but not recursively.
    assert of_mock.called == 1
    # We shouldn't convert the whole array, nor the dict here
    assert value.index(0).is_lazy()
    assert of_mock.called == 2
    assert value.index(1).is_lazy()
    assert of_mock.called == 3
    assert value.index(2).is_lazy()
    assert of_mock.called == 4

    # piercing into the lazy value, should be called twice
    # But we get away with only one additional call here (???)
    assert value.index(0).access("a")
    assert of_mock.called == 5


def test_laziness_with_object_access(monkeypatch):
    original_of = RuntimeValue.of

    def of_mock(*args, **kwargs):
        of_mock.called += 1
        return original_of(*args, **kwargs)

    of_mock.called = 0
    monkeypatch.setattr(RuntimeValue, "of", staticmethod(of_mock))
    value = RuntimeValue.of({"a": {"b": 1}, "b": {"b": 2}, "c": {"b": 3}}, lazy=True)
    assert of_mock.called == 1
    assert value.access("a").is_lazy()
    assert of_mock.called == 2
    # I have no idea why we get away with only one additional call here
    assert value.access("a").access("b").to_python() == 1
    assert of_mock.called == 3


def test_laziness_with_array_length(monkeypatch):
    original_of = RuntimeValue.of

    def of_mock(*args, **kwargs):
        of_mock.called += 1
        return original_of(*args, **kwargs)

    of_mock.called = 0
    monkeypatch.setattr(RuntimeValue, "of", staticmethod(of_mock))
    value = RuntimeValue.of([{"a": 1}, {"a": 2}, {"a": 3}], lazy=True)
    assert of_mock.called == 1
    assert len(value) == 3
    assert of_mock.called == 1


def test_laziness_with_object_keys(monkeypatch):
    original_of = RuntimeValue.of

    def of_mock(*args, **kwargs):
        of_mock.called += 1
        return original_of(*args, **kwargs)

    of_mock.called = 0
    monkeypatch.setattr(RuntimeValue, "of", staticmethod(of_mock))
    value = RuntimeValue.of({"a": 1, "b": 2, "c": 3}, lazy=True)
    assert of_mock.called == 1
    assert list(value.keys()) == ["a", "b", "c"]
    assert of_mock.called == 1


def test_resuse_full_value_when_already_evaluated_arrays(monkeypatch):
    original_of = RuntimeValue.of

    def of_mock(*args, **kwargs):
        of_mock.called += 1
        return original_of(*args, **kwargs)

    of_mock.called = 0
    monkeypatch.setattr(RuntimeValue, "of", staticmethod(of_mock))
    value = RuntimeValue.of([{"a": 1}, {"a": 2}, {"a": 3}], lazy=True)
    assert of_mock.called == 1
    assert len(value.value) == 3  # This forces the full value to be evaluated
    assert of_mock.called == 4

    # But since it's computed, we shouldn't get any additional calls
    assert value.index(0) is value.index(0)
    assert of_mock.called == 4


def test_resuse_full_value_when_already_evaluated_object(monkeypatch):
    original_of = RuntimeValue.of

    def of_mock(*args, **kwargs):
        of_mock.called += 1
        return original_of(*args, **kwargs)

    of_mock.called = 0

    monkeypatch.setattr(RuntimeValue, "of", staticmethod(of_mock))
    value = RuntimeValue.of({"a": 1, "b": 2, "c": 3}, lazy=True)
    assert of_mock.called == 1
    assert len(value.value) == 3  # This forces the full value to be evaluated
    assert of_mock.called == 4

    # But since it's computed, we shouldn't get any additional calls
    assert value.access("a") is value.access("a")
    assert of_mock.called == 4


# Thrashing tests (because we have two different caches)
def test_slice_and_value_thrashing():
    data = [{"a": 1}, {"a": 2}, {"a": 3}]
    # First, value.index first
    value = RuntimeValue.of(data, lazy=True)
    v1 = value.index(0, 2)
    v2 = value.value
    v3 = value.index(0, 2)
    v4 = value.value
    assert v1 == v3
    assert v2 == v4

    # Then, value.value first
    value = RuntimeValue.of(data, lazy=True)
    v1 = value.value
    v2 = value.index(0, 2)
    v3 = value.value
    v4 = value.index(0, 2)
    assert v1 == v3
    assert v2 == v4


def test_index_and_value_thrashing():
    data = [{"a": 1}, {"a": 2}, {"a": 3}]
    value = RuntimeValue.of(data, lazy=True)
    v1 = value.index(0)
    v2 = value.value
    v3 = value.index(0)
    v4 = value.value
    assert v1 == v3
    assert v2 == v4

    value = RuntimeValue.of(data, lazy=True)
    v1 = value.value
    v2 = value.index(0)
    v3 = value.value
    v4 = value.index(0)
    assert v1 == v3
    assert v2 == v4


def test_access_and_value_thrashing():
    data = {"a": 1, "b": 2, "c": 3}
    value = RuntimeValue.of(data, lazy=True)
    v1 = value.access("a")
    v2 = value.value
    v3 = value.access("a")
    v4 = value.value
    assert v1 == v3
    assert v2 == v4

    value = RuntimeValue.of(data, lazy=True)
    v1 = value.value
    v2 = value.access("a")
    v3 = value.value
    v4 = value.access("a")
    assert v1 == v3
    assert v2 == v4
