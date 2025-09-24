from mistql import MistQLInstance
import json
import os
import time


def test_performance():
    with open(os.path.join(os.path.dirname(__file__), "..", "shared", "data", "nobel-prizes.json"), "r") as f:
        nobel_prizes = json.load(f)
    mq = MistQLInstance(lazy=False)
    start_time = time.time()
    for i in range(5000):
      mq.query("count @.prizes", nobel_prizes)
    end_time = time.time()
    print(f"Time taken: {end_time - start_time} seconds")