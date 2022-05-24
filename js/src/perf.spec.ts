import mistql from ".";
import fs from "fs";
import assert from "assert";
import { performance } from 'perf_hooks';

// The following is scaffolding for performance testing.

type Summary = {
  median: number;
  mean: number;
  p95: number;
};

type Options = Partial<{
  iterations: number;
  name: string;
}>;

function time(fn, options: Options = {}) {
  const { iterations = 100, name } = options;
  const times = [];
  for (let i = 0; i < iterations; i++) {
    const start = performance.now();
    fn();
    const end = performance.now();
    times.push(end - start);
  }
  const summary = {
    median: times.sort()[Math.floor(times.length / 2)],
    mean: times.reduce((a, b) => a + b, 0) / times.length,
    p95: times.sort()[Math.floor(times.length * 0.95)],
  };
  const titleString = name ? ` for "${name}"` : "";
  console.log(`Performance Summary${titleString} (n=${iterations}):`);
  console.table(summary);
  return summary;
}

function assertTime(
  fn: () => void,
  predicate: (summary: Summary) => boolean,
  options: Options = {}
) {
  const summary = time(fn, options);
  assert(predicate(summary), `Expected ${JSON.stringify(summary)}`);
}

// The following is the actual test code.

describe("mistql performance", () => {
  describe("parsing", () => {
    it("should have query parsing performance similar to native json parsing", () => {
      const maxRatio = 100;

      const nobelPrizes = fs.readFileSync(
        __dirname + "/shared/data/nobel-prizes.json",
        "utf8"
      );

      const { median: nativeJSTime } = time(
        () => {
          JSON.parse(nobelPrizes);
        },
        { name: "native json parse", iterations: 20 }
      );

      const { median: mistqlTime } = time(
        () => {
          mistql.query(nobelPrizes, null);
        },
        { name: "mistql parse", iterations: 20 }
      );

      console.log("Ratio: ", mistqlTime / nativeJSTime);

      assert.ok(mistqlTime < nativeJSTime * maxRatio);
    }).timeout(10000);
  });

  describe("execution", () => {
    it("performs non-utf8 string indexing quickly", () => {
      const rfc6455 = fs.readFileSync(
        __dirname + "/shared/data/rfc6455.txt",
        "utf8"
      );
      assertTime(
        () => {
          mistql.query("@[10000]", rfc6455);
        },
        (summary) => summary.median < 2
      );
    });

    it("performs utf-8 string indexing quickly", () => {
      const rfc6455 =
        "👋🏽" + fs.readFileSync(__dirname + "/shared/data/rfc6455.txt", "utf8");

      assertTime(
        () => {
          mistql.query("@[10000]", rfc6455);
        },
        (summary) => summary.median < 2
      );
    });

    it("handles efficient rtts", () => {
      // This will likely differ on different systems, we should make it wide
      // enough to mostly work on whatever systems we need.
      const nobelPrizes = JSON.parse(
        fs.readFileSync(__dirname + "/shared/data/nobel-prizes.json", "utf8")
      );

      assertTime(
        () => {
          mistql.query("@", nobelPrizes);
        },
        ({ median }) => median < 50
      );
    });

    it("has low fixed cost to do a query", () => {
      assertTime(
        () => {
          mistql.query("@", null);
        },
        ({ median }) => median < 0.05
      );
    });
  });
});
