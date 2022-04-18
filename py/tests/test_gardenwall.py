from math import nan, inf
from mistql.gardenwall import input_garden_wall
from mistql.runtime_value import RuntimeValue


def test_nan_converts_to_null_value():
    assert input_garden_wall(nan) == RuntimeValue.of(None)


def test_inf_converts_to_null_value():
    assert input_garden_wall(inf) == RuntimeValue.of(None)


def test_neg_inf_converts_to_null_value():
    assert input_garden_wall(-inf) == RuntimeValue.of(None)
