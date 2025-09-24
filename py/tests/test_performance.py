from mistql import MistQLInstance
import json
import os
import time
import cProfile
import pstats
import io


def test_performance():
    with open(os.path.join(os.path.dirname(__file__), "..", "shared", "data", "nobel-prizes.json"), "r") as f:
        nobel_prizes = json.load(f)
    mq = MistQLInstance(lazy=True)
    
    # Profile the performance test
    profiler = cProfile.Profile()
    profiler.enable()
    
    for i in range(1000):
      mq.query("count @.prizes", nobel_prizes)
    
    profiler.disable()
    # Print profiling results
    s = io.StringIO()
    ps = pstats.Stats(profiler, stream=s).sort_stats('cumulative')
    ps.dump_stats("profile.pstats")
