import assert from "assert";
import { execute } from "../executor";
import { parseOrThrow } from "../parser";

describe("builtins", () => {
  describe("#map", () => {
    it("correctly maps simple values", () => {
      assert.deepStrictEqual(
        execute(parseOrThrow("map @ + 1 [1, 2, 3]"), {}),
        [2, 3, 4]
      );
    });

    it("correctly maps structy values", () => {
      assert.deepStrictEqual(
        execute(parseOrThrow("map (feature + 1) arr"), {
          arr: [{ feature: 1 }, { feature: 2 }, { feature: 3 }],
        }),
        [2, 3, 4]
      );
    });
  });

  describe("#filter", () => {
    it("correctly filters events", () => {
      assert.deepStrictEqual(
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

  describe("#reduce", () => {
    it("can sum", () => {
      assert.strictEqual(
        execute(parseOrThrow("reduce @[0] + @[1] 0 @"), [1, 4, 5, 7, 8]),
        1 + 4 + 5 + 7 + 8
      );
    });
  });

  describe("#mapvalues", () => {
    it("correctly changes the values of each", () => {
      assert.deepStrictEqual(
        execute(parseOrThrow("@ | mapvalues @ + 2"), {
          one: 1,
          two: 2,
          three: 3,
          four: 4,
        }),
        {
          one: 3,
          two: 4,
          three: 5,
          four: 6,
        }
      );
    });
  });

  describe("#filtervalues", () => {
    it("correctly filters values", () => {
      assert.deepStrictEqual(
        execute(parseOrThrow("@ | filtervalues @ > 2"), {
          one: 1,
          two: 2,
          three: 3,
          four: 4,
        }),
        { three: 3, four: 4 }
      );
    });
  });

  describe("#filterkeys", () => {
    it("correctly filters values", () => {
      assert.deepStrictEqual(
        execute(parseOrThrow('@ | filterkeys @ > "one"'), {
          one: 1,
          two: 2,
          three: 3,
          four: 4,
        }),
        { two: 2, three: 3 }
      );
    });
  });

  describe("#mapkeys", () => {
    it("correctly filters values", () => {
      assert.deepStrictEqual(
        execute(parseOrThrow('@ | mapkeys @ + "_old"'), {
          one: 1,
          two: 2,
          three: 3,
          four: 4,
        }),
        {
          one_old: 1,
          two_old: 2,
          three_old: 3,
          four_old: 4,
        }
      );
    });
  });

  describe("#plus", () => {
    it("works for numbers", () => {
      assert.strictEqual(execute(parseOrThrow("1 + -2"), {}), -1);
    });

    it("works for strings", () => {
      assert.strictEqual(execute(parseOrThrow('"sup "+ "bro"'), {}), "sup bro");
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
      assert.deepStrictEqual(
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

    it("correctly indexes strings", () => {
      assert.strictEqual(execute(parseOrThrow('"abcdefg" | index 2'), {}), "c");
    });

    it("allows ranges", () => {
      assert.strictEqual(
        execute(parseOrThrow('"abcdefg" | index 2 4'), {}),
        "cd"
      );
    });

    it("allows ranges with implicit end", () => {
      assert.strictEqual(
        execute(parseOrThrow('"abcdefg" | index 3 null'), {}),
        "defg"
      );
    });

    it("allows ranges with implicit start", () => {
      assert.strictEqual(
        execute(parseOrThrow('"abcdefg" | index null 3'), {}),
        "abc"
      );
    });

    it("allows ranges with arrays", () => {
      assert.deepStrictEqual(
        execute(parseOrThrow("[1, 2, 3, 4, 5] | index 1 3"), {}),
        [2, 3]
      );
    });

    it("allows ranges with negative indices", () => {
      assert.deepStrictEqual(
        execute(parseOrThrow("[1, 2, 3, 4, 5] | index (-3) (-1)"), {}),
        [3, 4]
      );
    });

    it("works with indexing syntax", () => {
      assert.strictEqual(execute(parseOrThrow("[1, 2, 3, 4, 5][0]"), {}), 1);
    });

    it("indexes from the back of an array given a negative", () => {
      assert.strictEqual(execute(parseOrThrow("[1, 2, 3, 4, 5][-2]"), {}), 4);
    });

    it("indexes from the back of an array given a negative number", () => {
      assert.strictEqual(execute(parseOrThrow("[1, 2, 3, 4, 5][-2]"), {}), 4);
    });

    it("indexes keys correctly", () => {
      assert.strictEqual(execute(parseOrThrow('{hi: 1}["hi"]'), {}), 1);
    });

    it("can index strings", () => {
      assert.strictEqual(execute(parseOrThrow('"hi"[0]'), {}), "h");
    });
  });

  describe("#keys", () => {
    it("correctly filters events", () => {
      assert.deepStrictEqual(
        execute(parseOrThrow("@ | keys"), { type: "hi", foo: 1 }),
        ["foo", "type"]
      );
    });
  });

  describe("#values", () => {
    it("correctly filters events", () => {
      assert.deepStrictEqual(
        execute(parseOrThrow("@ | values"), { type: 5, foo: 1 }),
        [1, 5]
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
      assert.deepStrictEqual(
        execute(parseOrThrow("events | groupby email"), { events }),
        expected
      );
    });

    it("groups non-strings", () => {
      const events = [
        { type: "signup", id: 1 },
        { type: "signup", id: 2 },
        { type: "play", id: 2 },
        { type: "play", id: 2 },
      ];
      const expected = {
        1: [
          {
            id: 1,
            type: "signup",
          },
        ],
        2: [
          {
            id: 2,
            type: "signup",
          },
          {
            id: 2,
            type: "play",
          },
          {
            id: 2,
            type: "play",
          },
        ],
      };
      assert.deepStrictEqual(
        execute(parseOrThrow("events | groupby id"), { events }),
        expected
      );
    });
  });

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

    it("compares structs correctly", () => {
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
});
