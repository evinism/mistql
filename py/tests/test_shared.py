import json
from typing import Any, List, Tuple

import pytest
from mistql import query

with open("shared/testdata.json") as f:
    testdata = json.load(f)


Case = Tuple[List[Tuple[str, Any, Any]], str, str, str]
cases: List[Tuple[List[Tuple[str, Any, Any]], str, str, str]] = []

for block in testdata["data"]:
    for innerblock in block["cases"]:
        for test in innerblock["cases"]:
            cases.append(
                (
                    [
                        (
                            assertion["query"], 
                            assertion["data"], 
                            assertion.get("expected"), 
                            assertion.get("throws")
                        ) for assertion in test["assertions"]
                    ],
                    block["describe"],
                    innerblock["describe"],
                    test["it"],
                )
            )


@pytest.mark.parametrize("case", cases, ids=lambda x: f"{x[1]}::{x[2]}::{x[3]}")
def test_shared(case: Case):
    for target_query, data, expected, throws in case[0]:
        if throws:
            with pytest.raises(Exception):
                query(target_query, data)
        else:
            assert query(target_query, data) == expected
