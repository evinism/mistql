import json
from typing import Any, List, Optional, Tuple

import pytest
from mistql import query

with open("shared/testdata.json", "rb") as f:
    testdata = json.load(f)


Case = Tuple[List[Tuple[str, Any, Any]], str, str, str, Optional[str]]
non_skipped_cases: List[Case] = []
skipped_cases: List[Case] = []


SELF_LANG_ID = "py"


for block in testdata["data"]:
    for innerblock in block["cases"]:
        for test in innerblock["cases"]:
            if SELF_LANG_ID in test.get("skip", []):
                target = skipped_cases
            else:
                target = non_skipped_cases
            target.append(
                (
                    [
                        (
                            assertion["query"],
                            assertion["data"],
                            assertion.get("expected"),
                            assertion.get("expectedSet"),
                            assertion.get("throws"),
                        )
                        for assertion in test["assertions"]
                    ],
                    block["describe"],
                    innerblock["describe"],
                    test["it"],
                    test.get("skip"),
                )
            )


def get_test_id_for_case(case: Case) -> str:
    return f"{case[1]}::{case[2]}::{case[3]}"


@pytest.mark.parametrize("case", non_skipped_cases, ids=get_test_id_for_case)
def test_shared(case: Case):
    for target_query, data, expected, expectedSet, throws in case[0]:
        if throws:
            with pytest.raises(Exception):
                query(target_query, data)
        elif expectedSet:
            assert query(target_query, data) in expectedSet
        else:
            assert query(target_query, data) == expected


if len(skipped_cases) > 0:

    @pytest.mark.skip
    @pytest.mark.parametrize("case", skipped_cases, ids=get_test_id_for_case)
    def test_shared_skipped(case: Case):
        pass
