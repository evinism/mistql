import assert from "assert";
import { query } from ".";

describe("index", () => {
  describe("#query", () => {
    it("functions on a very basic level", () => {
      assert.deepStrictEqual(query("@ | map @ + 1", [1, 2, 3]), [2, 3, 4]);
    });

    it("supports being passed falsy values for keys", () => {
      assert.deepStrictEqual(query("key + 1", { key: 0 }), 1);
    });

    it("allows a bunch unary operators in a row", () => {
      assert.deepStrictEqual(query("!!!!arg", { arg: true }), true);
      assert.deepStrictEqual(query("!!!!!true", { arg: true }), false);
      assert.deepStrictEqual(query("-num", { num: 0.5 }), -0.5);
      assert.deepStrictEqual(query("-5", {}), -5);
      assert.deepStrictEqual(query("--5", {}), 5);
    });
  });
});
