import assert from 'assert';
import { makeIndicator } from "./errors";

describe("errors", () => {
  describe("#makeIndicator", () => {
    it("works on a very simple string with a very simple position", () => {
      const expected = "hello there\n^";
      assert.strictEqual(makeIndicator('hello there', 0), expected);
    });

    it("works on a simple string somewhere else", () => {
      const expected = "hello there\n     ^";
      assert.strictEqual(makeIndicator('hello there', 5), expected);
    });

    it("works on a string with a tab", () => {
      const expected = "hello\tthere\n     \t^";
      assert.strictEqual(makeIndicator('hello\tthere', 6), expected);
    });

    it("works on a string with many lines", () => {
      const expected = "hello\n  ^\nthere";
      assert.strictEqual(makeIndicator('hello\nthere', 2), expected);
    });
  });
})