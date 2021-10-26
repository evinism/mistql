import assert from "assert";
import { query } from ".";

// A random grab-bag of integration tests

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

    it("doesn't allow object access of inherited properties", () => {
      assert.strictEqual(query("@.length", [1, 2, 3]), null);
      assert.strictEqual(query("@.map", [1, 2, 3]), null);
      assert.strictEqual(query('(regex "hi").lastIndex', [1, 2, 3]), null);
    });

    it("allows complex expressions as part of object and array literals", () => {
      assert.deepStrictEqual(
        query('([-1, { isSpot: dog == "spot"}])', { dog: "spot" }),
        [-1, { isSpot: true }]
      );
    });
  });

  describe("indexing", () => {
    it("correctly grabs the first element", () => {
      assert.strictEqual(query("[1, 2, 3, 4, 5][0]", {}), 1);
    });

    it("returns null for empty arrays", () => {
      assert.strictEqual(query("[][0]", {}), null);
      assert.strictEqual(query("[][-1]", {}), null);
    });

    it("correctly grabs the last element", () => {
      assert.strictEqual(query("[1, 2, 3, 4, 5][-1]", {}), 5);
    });

    it("slices with first param missing", () => {
      assert.deepStrictEqual(query("[1, 2, 3, 4, 5][:-1]", {}), [1, 2, 3, 4]);
      assert.deepStrictEqual(query("[1, 2, 3, 4, 5][:3]", {}), [1, 2, 3]);
    });

    it("slices with second param missing", () => {
      assert.deepStrictEqual(query("[1, 2, 3, 4, 5][-2:]", {}), [4, 5]);
      assert.deepStrictEqual(query("[1, 2, 3, 4, 5][2:]", {}), [3, 4, 5]);
    });

    it("slices with first and last params", () => {
      assert.deepStrictEqual(query("[1, 2, 3, 4, 5][1:-1]", {}), [2, 3, 4]);
      assert.deepStrictEqual(query("[1, 2, 3, 4, 5][2:4]", {}), [3, 4]);
    });
  });

  describe("weird overwriting stuff", () => {
    it("doesn't overwrite binary operators", () => {
      assert.deepStrictEqual(
        query("1 + 1 - 1 * 1 / 1", { "+": 1, "*": 1, "-": 1, "/": 1 }),
        1
      );
    });

    it("doesn't overwrite indexing", () => {
      assert.deepStrictEqual(query("[1, 2, 3][1]", { index: "hello" }), 2);
    });

    it("allows us to filter on values that overlap with builtin names", () => {
      assert.deepStrictEqual(
        query('[{filter: "hp"}, {filter: "lp"}] | filter filter == "lp"', {
          index: "hello",
        }),
        [{ filter: "lp" }]
      );
    });

    it("gives us a method for un-overwriting", () => {
      assert.deepStrictEqual(
        query('[{filter: hp}, {filter: lp}] | $.filter filter == "lp"', {
          filter: "hello",
          hp: "hp",
          lp: "lp",
        }),
        [{ filter: "lp" }]
      );
    });

    it("gives us a method for un-overwriting things in the data param", () => {
      assert.deepStrictEqual(
        query('[{filter: hp}, {filter: lp}] | $.filter filter == $.@.filter',
          { filter: "lp", lp: "lp", hp: "hp" }),
        [{ filter: "lp" }]
      );
    });

    it("supports truth tables", () => {
      assert.equal(query('!!{}', null), false);
      assert.equal(query('!!{foo: "bar"}', null), true);
      assert.equal(query('!![]', null), false);
      assert.equal(query('!![0]', null), true);
      assert.equal(query('!!""', null), false);
      assert.equal(query('!!"hi"', null), true);
      assert.equal(query('!!0', null), false);
      assert.equal(query('!!1', null), true);
      assert.equal(query('!!(regex "")', null), true);
      assert.equal(query('!!(regex "hi")', null), true);
      assert.equal(query('!!(regex "hi")', null), true);
      assert.equal(query('!!float', null), true);
    });
  });
});
