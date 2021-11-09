import assert from "assert";
import { execute } from "../executor";
import { parseOrThrow } from "../parser";

describe("builtins", () => {
  /* THESE SHOULD ALL BE TRANSFERRED OVER TO CROSS-IMPLEMENTATION TESTS */
  /* All tests below have not yet been transferred over */

  describe("#sort", () => {
    it("sensibly sorts numbers", () => {
      assert.deepStrictEqual(
        execute(parseOrThrow("[11, 2, 32, 104, 5] | sort"), {}),
        [2, 5, 11, 32, 104]
      );
    });

    it("sensibly sorts strings", () => {
      assert.deepStrictEqual(
        execute(
          parseOrThrow('["banana", "apple", "carrot", "cabbage"] | sort'),
          {}
        ),
        ["apple", "banana", "cabbage", "carrot"]
      );
    });

    it("sensibly sorts booleans", () => {
      assert.deepStrictEqual(
        execute(parseOrThrow("[true, false, true, false] | sort"), {}),
        [false, false, true, true]
      );
    });
  });

  describe("#sortby", () => {
    it("sensibly sorts items based on the specified expression", () => {
      assert.deepStrictEqual(
        execute(parseOrThrow("items | sortby sk"), {
          items: [{ sk: 11 }, { sk: 2 }, { sk: 32 }, { sk: 104 }, { sk: 5 }],
        }),
        [{ sk: 2 }, { sk: 5 }, { sk: 11 }, { sk: 32 }, { sk: 104 }]
      );
    });
  });

  describe("#reverse", () => {
    it("handles empty arrays", () => {
      assert.deepStrictEqual(execute(parseOrThrow("[] | reverse"), {}), []);
    });

    it("reverses arrays", () => {
      assert.deepStrictEqual(
        execute(parseOrThrow("[1, 2, 3, 4, 5] | reverse"), {}),
        [5, 4, 3, 2, 1]
      );
    });
  });

  describe("#head", () => {
    it("handles empty arrays", () => {
      assert.deepStrictEqual(execute(parseOrThrow("[][:3]"), {}), []);
    });

    it("grabs the first n elements", () => {
      assert.deepStrictEqual(
        execute(parseOrThrow("[1, 2, 3, 4, 5][:3]"), {}),
        [1, 2, 3]
      );
    });
  });

  describe("#tail", () => {
    it("handles empty arrays", () => {
      assert.deepStrictEqual(execute(parseOrThrow("[][:3]"), {}), []);
    });

    it("grabs the last n elements", () => {
      assert.deepStrictEqual(
        execute(parseOrThrow("[1, 2, 3, 4, 5][-3:]"), {}),
        [3, 4, 5]
      );
    });
  });

  describe("#sum", () => {
    it("makes empty arrays sum to zero", () => {
      assert.strictEqual(execute(parseOrThrow("[] | sum"), {}), 0);
    });

    it("grabs the first n elements", () => {
      assert.strictEqual(
        execute(parseOrThrow("[1, 2, 3, 4, 5] | sum"), {}),
        15
      );
    });
  });

  describe("#if", () => {
    it("chooses the first on truthy values", () => {
      assert.strictEqual(
        execute(parseOrThrow("if arg left right"), {
          arg: 1,
          left: 2,
          right: 3,
        }),
        2
      );
    });

    it("chooses the second on falsy values", () => {
      assert.strictEqual(
        execute(parseOrThrow("if argz left right"), {
          argz: 0,
          left: 2,
          right: 3,
        }),
        3
      );
    });
  });

  describe("#dotaccessor", () => {
    it("correctly parses deep values", () => {
      assert.deepStrictEqual(
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

    it("returns null on unknown value", () => {
      assert.deepStrictEqual(
        execute(parseOrThrow("@.hi"), {
          hello: { there: {} },
        }),
        null
      );
    });

    it("has automatic null coalescing", () => {
      assert.deepStrictEqual(
        execute(parseOrThrow("hello.there.hows.it.going"), {
          hello: { there: {} },
        }),
        null
      );
    });
  });

  describe("#log", () => {
    it("passes values through", () => {
      assert.deepStrictEqual(execute(parseOrThrow('log {bleep: "hi"}'), null), {
        bleep: "hi",
      });
    });
  });

  describe("#summarize", () => {
    it("summarizes values", () => {
      assert.deepStrictEqual(
        execute(parseOrThrow("@ | summarize"), [1, 2, 5, 10, 12]),
        {
          min: 1,
          max: 12,
          mean: 6,
          median: 5,
          stddev: 4.33589667773576,
          variance: 18.8,
        }
      );
    });
  });

  describe("#sequence", () => {
    it("summarizes values", () => {
      const e = (type: string, data: string) => ({ type, data });
      assert.deepStrictEqual(
        execute(parseOrThrow('@ | sequence type=="chat" type == "convert"'), [
          e("convert", "one"),
          e("chat", "two"),
          e("convert", "three"),
          e("convert", "four"),
          e("chat", "five"),
          e("convert", "six"),
        ]),
        [
          [e("chat", "two"), e("convert", "three")],
          [e("chat", "two"), e("convert", "four")],
          [e("chat", "two"), e("convert", "six")],
          [e("chat", "five"), e("convert", "six")],
        ]
      );
    });
  });

  describe("#replace", () => {
    it("correctly replaces string values", () => {
      assert.strictEqual(
        execute(parseOrThrow('"hello" | replace "ll" "zop"'), null),
        "hezopo"
      );
    });

    it("inteprets the first arg as a regex", () => {
      assert.strictEqual(
        execute(parseOrThrow('"hello" | replace (regex "l+") "zop"'), null),
        "hezopo"
      );
    });

    it("allows flags", () => {
      assert.strictEqual(
        execute(parseOrThrow('"hello" | replace (regex "l" "g") "za"'), null),
        "hezazao"
      );
    });

    it("replaces only first instance by default", () => {
      assert.strictEqual(
        execute(parseOrThrow('"hello" | replace "l" "za"'), null),
        "hezalo"
      );
    });
  });

  describe("#match/#regex", () => {
    it("correctly matches string values", () => {
      assert.strictEqual(
        execute(parseOrThrow('"hello" | match (regex "he..o")'), null),
        true
      );
      assert.strictEqual(
        execute(parseOrThrow('"hello" | match (regex "he..a")'), null),
        false
      );
    });

    it("allows flags", () => {
      assert.strictEqual(
        execute(parseOrThrow('"Hello" | match (regex "[a-z]ello")'), null),
        false
      );
      assert.strictEqual(
        execute(parseOrThrow('"Hello" | match (regex "[a-z]ello" "i")'), null),
        true
      );
    });

    it("allows matching via the =~ operator", () => {
      assert.strictEqual(
        execute(parseOrThrow('"hello" =~ (regex "he..o")'), null),
        true
      );
      assert.strictEqual(
        execute(parseOrThrow('"hello" =~ (regex "he..a")'), null),
        false
      );
    });

    it("doesn't reverse the underlying AST when using the =~ operator", () => {
      assert.deepStrictEqual(
        execute(parseOrThrow('[1, 2] | map "hello" =~ (regex "he..o")'), null),
        [true, true]
      );
    });
  });

  describe("#equal", () => {
    it("compares strings correctly", () => {
      assert.strictEqual(
        execute(parseOrThrow('"hello" == "hello"'), null),
        true
      );
      assert.strictEqual(
        execute(parseOrThrow('"hello" == "hellz"'), null),
        false
      );
    });

    it("compares numbers correctly", () => {
      assert.strictEqual(execute(parseOrThrow("1 == 1"), null), true);
      assert.strictEqual(execute(parseOrThrow("1 == -1"), null), false);
    });

    it("compares null correctly", () => {
      assert.strictEqual(execute(parseOrThrow("null == null"), null), true);
    });

    it("returns false for different types", () => {
      assert.strictEqual(execute(parseOrThrow("0 == null"), null), false);
      assert.strictEqual(execute(parseOrThrow("null == 0"), null), false);
      assert.strictEqual(execute(parseOrThrow('null == "null"'), null), false);
      assert.strictEqual(execute(parseOrThrow('0 == "0"'), null), false);
      assert.strictEqual(execute(parseOrThrow("1 == true"), null), false);
      assert.strictEqual(execute(parseOrThrow("0 == false"), null), false);
      assert.strictEqual(
        execute(parseOrThrow('"false" == false'), null),
        false
      );
      assert.strictEqual(execute(parseOrThrow('"true" == true'), null), false);
    });

    it("compares arrays correctly", () => {
      assert.strictEqual(
        execute(parseOrThrow("[1, 2, 3] == [1, 2, 3]"), null),
        true
      );
      assert.strictEqual(execute(parseOrThrow("[] == []"), null), true);
      assert.strictEqual(execute(parseOrThrow("[[]] == [[]]"), null), true);
      assert.strictEqual(
        execute(parseOrThrow('[["hi"]] == [["hi"]]'), null),
        true
      );
      assert.strictEqual(execute(parseOrThrow("[1, 2] == [1]"), null), false);
      assert.strictEqual(
        execute(parseOrThrow('[["hi"]] == [["hz"]]'), null),
        false
      );
      assert.strictEqual(
        execute(parseOrThrow("[1, 2, 3] == [1, 2, 3, 4]"), null),
        false
      );
      assert.strictEqual(
        execute(parseOrThrow("[1, 2, 3] == [1, 2, 4]"), null),
        false
      );
    });

    it("compares objects correctly", () => {
      assert.strictEqual(
        execute(
          parseOrThrow(
            "{one: 1, two: 2, three: 3} == {one: 1, two: 2, three: 3}"
          ),
          null
        ),
        true
      );
      assert.strictEqual(execute(parseOrThrow("{} == {}"), null), true);
      assert.strictEqual(
        execute(
          parseOrThrow(
            "{one: 1, two: 2, three: 3} == {one: 1, two: 2, three: 4}"
          ),
          null
        ),
        false
      );
      assert.strictEqual(
        execute(
          parseOrThrow(
            "{one: 1, two: 2, three: 3} == {one: 1, two: 2, four: 3}"
          ),
          null
        ),
        false
      );
    });

    it("compares regexes correctly", () => {
      assert.strictEqual(
        execute(parseOrThrow('(regex "blah") == (regex "blah")'), null),
        true
      );
      assert.strictEqual(
        execute(parseOrThrow('(regex "blah" "g") == (regex "blah" "g")'), null),
        true
      );
      assert.strictEqual(
        execute(parseOrThrow('(regex "blah" "g") == (regex "blah")'), null),
        false
      );
      assert.strictEqual(
        execute(parseOrThrow('(regex "blah") == (regex "bl.h")'), null),
        false
      );
      assert.strictEqual(
        execute(parseOrThrow('(regex "bah" "g") == (regex "blah")'), null),
        false
      );
    });

    it("compares booleans correctly", () => {
      assert.strictEqual(execute(parseOrThrow("true == true"), null), true);
      assert.strictEqual(execute(parseOrThrow("false == false"), null), true);
      assert.strictEqual(execute(parseOrThrow("true == false"), null), false);
      assert.strictEqual(execute(parseOrThrow("false == true"), null), false);
    });
  });

  describe("#split", () => {
    it("splits basic strings", () => {
      assert.deepStrictEqual(
        execute(parseOrThrow('split "hi" @'), "ahi tuna"),
        ["a", " tuna"]
      );
    });

    it("splits via regexes", () => {
      assert.deepStrictEqual(
        execute(parseOrThrow('split (regex "h.") @'), "ahi tuna"),
        ["a", " tuna"]
      );
    });

    it("splits everywhere even with a non global regex", () => {
      assert.deepStrictEqual(
        execute(parseOrThrow('split (regex "a") @'), "babab"),
        ["b", "b", "b"]
      );
    });
  });

  describe("#join", () => {
    it("joins basic arrays", () => {
      assert.deepStrictEqual(
        execute(parseOrThrow('join "" @'), ["bo", "ba"]),
        "boba"
      );
    });

    it("joins with a delimiter", () => {
      assert.deepStrictEqual(
        execute(parseOrThrow('join ", " @'), ["bo", "ba"]),
        "bo, ba"
      );
    });

    it("round-trips with split", () => {
      assert.deepStrictEqual(
        execute(parseOrThrow('(split ":" @ | join ":") == @'), "hello:hi:sup"),
        true
      );
    });
  });

  describe("#entries", () => {
    it("splits objects into entries", () => {
      assert.deepStrictEqual(
        execute(parseOrThrow("entries @"), { a: 1, b: 2 }),
        [
          ["a", 1],
          ["b", 2],
        ]
      );
    });

    it("roundtrips with fromentries", () => {
      assert.deepStrictEqual(
        execute(parseOrThrow("(@ | entries | fromentries) == @"), {
          a: 1,
          b: 2,
        }),
        true
      );
    });
  });

  describe("#fromentries", () => {
    it("splits objects into entries", () => {
      assert.deepStrictEqual(
        execute(parseOrThrow("fromentries @"), [
          ["a", 1],
          ["b", 2],
        ]),
        { a: 1, b: 2 }
      );
    });

    it("roundtrips with entries", () => {
      assert.deepStrictEqual(
        execute(parseOrThrow("(@ | fromentries | entries) == @"), [
          ["a", 1],
          ["b", 2],
        ]),
        true
      );
    });

    it("uses nulls for missing values", () => {
      assert.deepStrictEqual(
        execute(parseOrThrow("fromentries @"), [[], ["b"], ["c", 1]]),
        {
          null: null,
          b: null,
          c: 1,
        }
      );
    });

    it("casts non-string keys to strings", () => {
      assert.deepStrictEqual(
        execute(parseOrThrow("fromentries @"), [
          [{}, 1],
          [[1, 2, 3], 2],
        ]),
        {
          "{}": 1,
          "[1,2,3]": 2,
        }
      );
    });
  });

  describe("#string", () => {
    it("casts values to strings", () => {
      assert.deepStrictEqual(execute(parseOrThrow("string @"), "hi"), "hi");
      assert.deepStrictEqual(execute(parseOrThrow("string @"), 1.5), "1.5");
      assert.deepStrictEqual(execute(parseOrThrow("string @"), null), "null");
      assert.deepStrictEqual(
        execute(parseOrThrow("string @"), [1, 2]),
        "[1,2]"
      );
      assert.deepStrictEqual(execute(parseOrThrow("string @"), true), "true");
      assert.deepStrictEqual(execute(parseOrThrow("string @"), false), "false");
    });
  });

  describe("#float", () => {
    it("casts values to floats", () => {
      assert.deepStrictEqual(execute(parseOrThrow("float @"), "1.5"), 1.5);
      assert.deepStrictEqual(
        execute(parseOrThrow("float @"), "10000.4"),
        10000.4
      );
      assert.deepStrictEqual(
        Number.isNaN(execute(parseOrThrow("float @"), "lol[]")),
        true
      );
      assert.throws(() => execute(parseOrThrow("float @"), []));
      assert.throws(() => execute(parseOrThrow("float @"), {}));
      assert.deepStrictEqual(execute(parseOrThrow("float @"), null), 0);
      assert.deepStrictEqual(execute(parseOrThrow("float @"), true), 1);
      assert.deepStrictEqual(execute(parseOrThrow("float @"), false), 0);
    });
  });

  describe("#apply", () => {
    it("allows easy modification to a value via piping", () => {
      assert.deepStrictEqual(execute(parseOrThrow("@ | apply @ + 1"), 1), 2);
      assert.deepStrictEqual(
        execute(parseOrThrow("@ | apply @ + 1 | apply @ + 1"), 1),
        3
      );
    });
    it("defines intermediate variables from the context variable", () => {
      assert.deepStrictEqual(
        execute(parseOrThrow("@ | apply a"), { a: 1, b: 2 }),
        1
      );
      assert.deepStrictEqual(
        execute(parseOrThrow("@ | apply b"), { a: 1, b: 2 }),
        2
      );
    });
  });
});
