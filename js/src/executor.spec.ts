import assert from "assert";
import { execute, inputGardenWall, outputGardenWall } from "./executor";
import { parseOrThrow } from "./parser";

describe("executor", () => {
  describe("#inputGardenWall", () => {
    const validInputs = [
      "hello",
      null,
      true,
      false,
      0,
      1,
      2,
      0.5,
      -0.5,
      [],
      ["cat"],
      ["cat", 5],
      { cat: "dog", dog: { eagle: "bat", hi: true } },
      [[[[[[["doug"]]]]]]],
    ];

    it("can handle any valid input type", () => {
      validInputs.forEach((item) => {
        assert.deepEqual(inputGardenWall(item), item);
      });
    });

    it("passes any valid input straight through", () => {
      validInputs.forEach((item) => {
        assert.deepEqual(execute(parseOrThrow("@"), item), item);
      });
    });

    it("coerces non-plain objects to plain objects", () => {
      assert.deepStrictEqual(inputGardenWall(Promise.resolve(5)), {});
      assert.deepStrictEqual(
        inputGardenWall(function () { }),
        {}
      );
    });

    it("coerces functions to blank objects", () => {
      assert.deepStrictEqual(inputGardenWall(() => { }), {});
    });

    it("coerces dates to timestamps", () => {
      assert.deepStrictEqual(inputGardenWall(new Date('10-10-2020')), JSON.parse(JSON.stringify(new Date('10-10-2020'))));
    });

    it("coerces object-numbers to numbers", () => {
      assert.deepStrictEqual(inputGardenWall(new Number(50)), 50);
    });

    it("coerces object-numbers to numbers", () => {
      assert.deepStrictEqual(inputGardenWall(new Boolean(true)), true);
    });

    it("coerces NaN to null internally", () => {
      assert.deepStrictEqual(inputGardenWall(NaN), null);
    });

    it("coerces infinity to null internally", () => {
      assert.deepStrictEqual(inputGardenWall(Infinity), null);
      assert.deepStrictEqual(inputGardenWall(-Infinity), null);
    });

    it("coerces object-number NaN to null internally", () => {
      assert.deepStrictEqual(inputGardenWall(new Number(NaN)), null);
    });

    it("coerces object-number infinity to null internally", () => {
      assert.deepStrictEqual(inputGardenWall(new Number(Infinity)), null);
      assert.deepStrictEqual(inputGardenWall(new Number(-Infinity)), null);
    });

    it("coerces own properties", () => {
      function foo() { }
      foo.hi = "doc";
      assert.deepEqual(inputGardenWall(foo), { hi: "doc" });
    });
  });

  describe("#outputGardenWall", () => {
    it("errors on function out", () => {
      assert.throws(() => outputGardenWall(function () { }));
    });

    it("errors on regex out", () => {
      assert.throws(() => outputGardenWall(/regex/));
    });
  });

  describe("#execute", () => {
    describe("references", () => {
      it("handles a simple reference", () => {
        const result = execute(parseOrThrow("foo"), { foo: 1 });
        assert.strictEqual(result, 1);
      });

      it("executes deep references", () => {
        const result = execute(parseOrThrow("foo.bar.baz"), {
          foo: { bar: { baz: 1 } },
        });
        assert.strictEqual(result, 1);
      });

      it("executes more complicated deep references", () => {
        execute(parseOrThrow("foo.bar.baz"), {
          foo: { bar: { baz: 1 }, bleep: 2 },
        });
      });

      it("throws on a bare missing reference", () => {
        assert.throws(() => execute(parseOrThrow("bar"), { foo: 1 }));
      });
    });

    describe("literals", () => {
      it("handles a simple string literal", () => {
        const result = execute(parseOrThrow('"foo"'), {});
        assert.strictEqual(result, "foo");
      });

      it("handles a simple number literal", () => {
        const result = execute(parseOrThrow("58320"), {});
        assert.strictEqual(result, 58320);
      });

      it("handles a simple null literal", () => {
        const result = execute(parseOrThrow("null"), {});
        assert.strictEqual(result, null);
      });

      it("handles a simple array literal", () => {
        const result = execute(parseOrThrow("[1, 2]"), {});
        assert.deepStrictEqual(result, [1, 2]);
      });

      it("handles an array literal with references", () => {
        const result = execute(parseOrThrow("[foo.bar, baz]"), {
          foo: { bar: 5 },
          baz: 6,
        });
        assert.deepStrictEqual(result, [5, 6]);
      });

      it("handles nested array literals", () => {
        const result = execute(parseOrThrow("[[foo.bar], baz]"), {
          foo: { bar: 5 },
          baz: 6,
        });
        assert.deepStrictEqual(result, [[5], 6]);
      });

      it("handles object literals", () => {
        const result = execute(parseOrThrow('{foo: "bar"}'), {});
        assert.deepStrictEqual(result, { foo: "bar" });
      });
    });

    describe("pipe", () => {
      it("handles piping to parameterized functions", () => {
        const result = execute(parseOrThrow("foo | map @ + 1"), {
          foo: [1, 2, 3, 4, 5],
        });
        assert.deepStrictEqual(result, [2, 3, 4, 5, 6]);
      });

      it("handles multiple pipes in a row", () => {
        const result = execute(parseOrThrow("foo | map @ + 1 | map @ * 2"), {
          foo: [1, 2, 3, 4, 5],
        });
        assert.deepStrictEqual(result, [4, 6, 8, 10, 12]);
      });

      it("operates left-associatively", () => {
        const result = execute(parseOrThrow("(foo | map @ + 1) | map @ * 2"), {
          foo: [1, 2, 3, 4, 5],
        });
        assert.deepStrictEqual(result, [4, 6, 8, 10, 12]);
      });

      it("correctly handles bare functions", () => {
        const result = execute(parseOrThrow("foo | count"), {
          foo: [1, 2, 3, 4, 5],
        });
        assert.deepStrictEqual(result, 5);
      });
    });
  });
});
