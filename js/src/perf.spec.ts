import mistql from ".";
import fs from "fs";
import assert from "assert";
import { performance } from 'perf_hooks';


function time(fn) {
  const start = performance.now();
  fn();
  return performance.now() - start;
}

describe("mistql performance", () => {
  describe("parsing", () => {
    it("should have query parsing performance similar to native json parsing", () => {
      // TODO: This is horrible in terms of performance!!
      // MistQL is performing at terrible levels, and we should aim to bring this down
      // by several orders of magnitude.
      const maxRatio = 2000;

      const nobelPrizes = fs.readFileSync(
        __dirname + "/shared/data/nobel-prizes.json",
        "utf8"
      );
  
      const nativeJSTime = time(() => {
        JSON.parse(nobelPrizes);
      });
  
      const mistqlTime = time(() => {
        mistql.query(nobelPrizes, null);
      });
  
      assert.ok(mistqlTime < nativeJSTime * maxRatio);
    }).timeout(10000);
  });

  describe("execution", () => {
    it("handles string indexing at roughly the same speed for non-utf8", () => {
      const maxRatio = 1.2;
      const rfc6455 = fs.readFileSync(
        __dirname + "/shared/data/rfc6455.txt",
        "utf8"
      );
      const nativeJSTime = time(() => {
        mistql.query("@", rfc6455)[10000]
      });
  
      const mistqlTime = time(() => {
        mistql.query("@[10000]", rfc6455);
      });
  
      assert.ok(mistqlTime < nativeJSTime * maxRatio);
    });

    it("handles string indexing within an order of magnitude for utf8", () => {
      // This can likely be tightened substantially if we move to an indexing
      // scheme that operates not off of constructing the entire array.
      const maxRatio = 20;
      const rfc6455 = "👋🏽" + fs.readFileSync(
        __dirname + "/shared/data/rfc6455.txt",
        "utf8"
      );

      const nativeJSTime = time(() => {
        mistql.query("@", rfc6455)[10000]
      });
  
      const mistqlTime = time(() => {
        mistql.query("@[10000]", rfc6455);
      });  
      assert.ok(mistqlTime < nativeJSTime * maxRatio);
    });

    it("handles efficient rtts", () => {
      // This will likely differ on different systems, we should make it wide
      // enough to mostly work on whatever systems we need.
      const maxDuration = 50;

      const nobelPrizes = JSON.parse(fs.readFileSync(
        __dirname + "/shared/data/nobel-prizes.json",
        "utf8"
      ));
      const mistqlTime = time(() => {
        mistql.query("@", nobelPrizes);
      });
      assert.ok(mistqlTime < maxDuration);
    });

    it("has low fixed cost to do a query", () => {
      const maxDuration = 1;
      const mistqlTime = time(() => {
        mistql.query("@", null);
      });
      assert.ok(mistqlTime < maxDuration);
    });
  });
});
