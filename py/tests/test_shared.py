import json
from typing import Any, List, Tuple

import pytest
from mistql import query

with open("shared/testdata.json") as f:
    testdata = json.load(f)


Testcase = Tuple[List[Tuple[str, Any, Any]], str, str]
testcases: List[Testcase] = []

for block in testdata["data"]:
    for test in block["cases"]:
        testcases.append(
            (
                [
                    (assertion["query"], assertion["data"], assertion["expected"])
                    for assertion in test["assertions"]
                ],
                block["describe"],
                test["it"],
            )
        )


@pytest.mark.parametrize("testcase", testcases, ids=lambda x: f"{x[1]}::{x[2]}")
def test_shared(testcase: Testcase):
    for target_query, data, expected in testcase[0]:
        assert query(target_query, data) == expected
