from mistql import MistQLInstance
import json
import os
import time
import cProfile
import pstats
import io
import random
from mistql.env_flags import PROFILE
import pytest


@pytest.mark.skipif(
    not PROFILE,
    reason="PROFILE is not set",
)
def test_performance():
    with open(
        os.path.join(
            os.path.dirname(__file__), "..", "shared", "data", "nobel-prizes.json"
        ),
        "r",
    ) as f:
        nobel_prizes = json.load(f)
    mq = MistQLInstance(lazy=True)
    nobel_prizes_len = len(nobel_prizes)

    # Profile the performance test
    profiler = cProfile.Profile()
    profiler.enable()

    ## Simple sparse query
    for i in range(1000):
        j = random.randint(0, nobel_prizes_len)
        mq.query("@.prizes[%s].motivation" % j, nobel_prizes)

    profiler.disable()

    # Print profiling results
    ps = pstats.Stats(profiler)
    ps.dump_stats("profile.pstats")
    ps.strip_dirs()
    ps.sort_stats("cumulative").print_stats(20)
