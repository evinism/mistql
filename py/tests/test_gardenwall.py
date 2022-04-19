from datetime import date, datetime, time
from math import inf, nan

from mistql.gardenwall import input_garden_wall
from mistql.runtime_value import RuntimeValue


def test_nan_converts_to_null_value():
    assert input_garden_wall(nan) == RuntimeValue.of(None)


def test_inf_converts_to_null_value():
    assert input_garden_wall(inf) == RuntimeValue.of(None)


def test_neg_inf_converts_to_null_value():
    assert input_garden_wall(-inf) == RuntimeValue.of(None)


def test_dates_convert_to_iso_string():
    assert input_garden_wall(date(2019, 1, 1)) == RuntimeValue.of("2019-01-01")


def test_times_convert_to_iso_string():
    assert input_garden_wall(time(12, 34, 56)) == RuntimeValue.of("12:34:56")


def test_datetimes_convert_to_iso_string():
    expected = RuntimeValue.of("2019-01-01T12:34:56")
    actual = input_garden_wall(datetime(2019, 1, 1, 12, 34, 56))
    assert actual == expected
