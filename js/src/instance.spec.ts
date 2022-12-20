import { MistQLInstance } from "./instance";
import assert from "assert";
import { jsFunctionToMistQLFunction } from "./util";

describe("Instance", () => {
  describe("#query", () => {
    it("should return the value of a simple query", () => {
      const instance = new MistQLInstance();
      assert.strictEqual(instance.query("hello", { hello: "there" }), "there");
    });
  });

  describe("extras", () => {
    it("should allow for basic extra functions", () => {
      const instance = new MistQLInstance({
        extras: {
          basicFunction: () => 1 + 2,
        },
      });
      assert.strictEqual(instance.query("@ | basicFunction", null), 3);
    });

    it("should allow complicated extra functions", () => {
      const sumargs = (args, stack, exec) =>
        args.map((arg) => exec(arg, stack)).reduce((a, b) => a + b, 0);
      const instance = new MistQLInstance({
        extras: {
          sumargs,
        },
      });
      assert.strictEqual(
        instance.query("sumargs 0 1 2 3 4 5 100 (-9)", null),
        106
      );
    });

    it("should accept usage of the jsFunctionToMistQLFunction method", () => {
      const instance = new MistQLInstance({
        extras: {
          intersperse: jsFunctionToMistQLFunction((a, b) =>
            a.flatMap((x) => [x, b]).slice(0, -1)
          ),
        },
      });
      assert.deepStrictEqual(
        instance.query("intersperse [1, 2, 3] @", "and a"),
        [1, "and a", 2, "and a", 3]
      );

      assert.throws(() => {
        instance.query("intersperse 1 2 3", null);
      });
    });
  });
});
