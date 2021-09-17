import assert from "assert";
import { execute } from "./executor";
import { parseOrThrow } from "./parser";

describe("builtins", () => {
  describe("#map", () => {
    it("correctly maps simple values", () => {
      assert.deepEqual(
        execute(parseOrThrow("map @ + 1 [1, 2, 3]"), {}),
        [2, 3, 4]
      );
    });

    it("correctly maps structy values", () => {
      assert.deepEqual(
        execute(parseOrThrow("map (feature + 1) arr"), {
          arr: [{ feature: 1 }, { feature: 2 }, { feature: 3 }],
        }),
        [2, 3, 4]
      );
    });
  });

  describe("#filter", () => {
    it("correctly filters events", () => {
      assert.deepEqual(
        execute(parseOrThrow('filter type == "hi" events'), {
          events: [
            { type: "hi", foo: 1 },
            { type: "there", foo: 2 },
          ],
        }),
        [{ type: "hi", foo: 1 }]
      );
    });
  });

  describe("#[numerical binary operators]", () => {
    it("works for +", () => {
      assert.strictEqual(execute(parseOrThrow("1 + 2"), {}), 3);
    });

    it("works for -", () => {
      assert.strictEqual(execute(parseOrThrow("1 - 2"), {}), -1);
    });

    it("works for *", () => {
      assert.strictEqual(execute(parseOrThrow("2 * 3"), {}), 6);
      assert.strictEqual(execute(parseOrThrow("-2 * 3"), {}), -6);
      assert.strictEqual(execute(parseOrThrow("-2 * -3"), {}), 6);
      assert.strictEqual(execute(parseOrThrow("2 * -3"), {}), -6);
    });

    it("works for %", () => {
      assert.strictEqual(execute(parseOrThrow("6 % 4"), {}), 2);
    });
  });

  describe("#[comparators]", () => {
    it("satisfies truth tables for >", () => {
      assert.strictEqual(execute(parseOrThrow("2 > 1"), {}), true);
      assert.strictEqual(execute(parseOrThrow("1 > 2"), {}), false);
      assert.strictEqual(execute(parseOrThrow("1 > 1"), {}), false);
    });

    it("satisfies truth tables for >=", () => {
      assert.strictEqual(execute(parseOrThrow("2 >= 1"), {}), true);
      assert.strictEqual(execute(parseOrThrow("1 >= 2"), {}), false);
      assert.strictEqual(execute(parseOrThrow("1 >= 1"), {}), true);
    });

    it("satisfies truth tables for <", () => {
      assert.strictEqual(execute(parseOrThrow("2 < 1"), {}), false);
      assert.strictEqual(execute(parseOrThrow("1 < 2"), {}), true);
      assert.strictEqual(execute(parseOrThrow("1 < 1"), {}), false);
    });

    it("satisfies truth tables for <=", () => {
      assert.strictEqual(execute(parseOrThrow("2 <= 1"), {}), false);
      assert.strictEqual(execute(parseOrThrow("1 <= 2"), {}), true);
      assert.strictEqual(execute(parseOrThrow("1 <= 1"), {}), true);
    });
  });

  describe("#find", () => {
    it("correctly finds events", () => {
      assert.deepEqual(
        execute(parseOrThrow('find type == "there" events'), {
          events: [
            { type: "hi", foo: 1 },
            { type: "there", foo: 1 },
            { type: "there", foo: 2 },
          ],
        }),
        { type: "there", foo: 1 }
      );
    });

    it("returns null if nothng satisfies", () => {
      assert.strictEqual(
        execute(parseOrThrow("@ | find @ == 4"), [1, 2, 3]),
        null
      );
    });
  });

  describe("#index", () => {
    it("correctly indexes arrays", () => {
      assert.strictEqual(
        execute(parseOrThrow("[1, 2, 3, 4, 5] | index 2"), {}),
        3
      );
    });

    it("returns null if out of bounds", () => {
      assert.strictEqual(
        execute(parseOrThrow("[1, 2, 3, 4, 5] | index 10"), {}),
        null
      );
    });
  });

  describe("#first", () => {
    it("correctly grabs the first element", () => {
      assert.strictEqual(
        execute(parseOrThrow("[1, 2, 3, 4, 5] | first"), {}),
        1
      );
    });

    it("returns null for empty arrays", () => {
      assert.strictEqual(execute(parseOrThrow("[] | first"), {}), null);
    });
  });

  describe("#last", () => {
    it("correctly grabs the last element", () => {
      assert.strictEqual(
        execute(parseOrThrow("[1, 2, 3, 4, 5] | last"), {}),
        5
      );
    });

    it("returns null for empty arrays", () => {
      assert.strictEqual(execute(parseOrThrow("[] | last"), {}), null);
    });
  });

  describe("#keys", () => {
    it("correctly filters events", () => {
      assert.deepEqual(
        execute(parseOrThrow("@ | keys"), { type: "hi", foo: 1 }),
        ["type", "foo"]
      );
    });
  });

  describe("#values", () => {
    it("correctly filters events", () => {
      assert.deepEqual(
        execute(parseOrThrow("@ | values"), { type: 5, foo: 1 }),
        [5, 1]
      );
    });
  });

  describe("#groupby", () => {
    it("correctly groups events", () => {
      const events = [
        { type: "signup", email: "test1@example.com" },
        { type: "signup", email: "test2@example.com" },
        { type: "play", email: "test2@example.com" },
        { type: "play", email: "test2@example.com" },
      ];
      const expected = {
        "test1@example.com": [
          {
            email: "test1@example.com",
            type: "signup",
          },
        ],
        "test2@example.com": [
          {
            email: "test2@example.com",
            type: "signup",
          },
          {
            email: "test2@example.com",
            type: "play",
          },
          {
            email: "test2@example.com",
            type: "play",
          },
        ],
      };
      assert.deepEqual(
        execute(parseOrThrow("events | groupby email"), { events }),
        expected
      );
    });
  });

  describe("#sort", () => {
    it("sensibly sorts numbers", () => {
      assert.deepEqual(
        execute(parseOrThrow("[11, 2, 32, 104, 5] | sort"), {}),
        [2, 5, 11, 32, 104]
      );
    });

    it("sensibly sorts strings", () => {
      assert.deepEqual(
        execute(
          parseOrThrow('["banana", "apple", "carrot", "cabbage"] | sort'),
          {}
        ),
        ["apple", "banana", "cabbage", "carrot"]
      );
    });

    it("sensibly sorts booleans", () => {
      assert.deepEqual(
        execute(parseOrThrow("[true, false, true, false] | sort"), {}),
        [false, false, true, true]
      );
    });
  });

  describe("#reverse", () => {
    it("handles empty arrays", () => {
      assert.deepEqual(execute(parseOrThrow("[] | reverse"), {}), []);
    });

    it("reverses arrays", () => {
      assert.deepEqual(
        execute(parseOrThrow("[1, 2, 3, 4, 5] | reverse"), {}),
        [5, 4, 3, 2, 1]
      );
    });
  });

  describe("#dotaccessor", () => {
    it("correctly parses deep values", () => {
      assert.deepEqual(
        execute(parseOrThrow("hello.over.there"), {
          hello: { over: { there: "hi" } },
        }),
        "hi"
      );
    });

    it("fails if the rhs isnt a reference", () => {
      // Might want to loosen this restriction at some point.
      assert.throws(() =>
        execute(parseOrThrow("hello.200"), {
          hello: { 200: "hi" },
        })
      );
    });

    it("allows complex lhs expressions", () => {
      assert.throws(
        () =>
          execute(parseOrThrow("(@ | second).apple"), [
            { apple: "sup" },
            { apple: "hi" },
          ]),
        "hi"
      );
    });
  });
});
