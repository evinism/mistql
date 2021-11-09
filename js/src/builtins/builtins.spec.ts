import assert from "assert";
import { execute } from "../executor";
import { parseOrThrow } from "../parser";


describe("builtins", () => {
  /* THESE SHOULD ALL BE TRANSFERRED OVER TO CROSS-IMPLEMENTATION TESTS */
  /* All tests below have not yet been transferred over */

  describe("#dotaccessor", () => {
    it("fails if the rhs isnt a reference", () => {
      // Might want to loosen this restriction at some point.
      assert.throws(() =>
        execute(parseOrThrow("hello.200"),
          {
            hello: {
              200: "hi"
            },
          })
      );
    });
  });


  describe("#match/#regex", () => {
    it("doesn't reverse the underlying AST when using the =~ operator", () => {
      assert.deepStrictEqual(
        execute(parseOrThrow('[1, 2] | map "hello" =~ (regex "he..o")'), null),
        [
          true,
          true
        ]
      );
    });
  });


  describe("#split", () => {
    it("splits basic strings", () => {
      assert.deepStrictEqual(
        execute(parseOrThrow('split "hi" @'),
          "ahi tuna"),
        [
          "a",
          " tuna"
        ]
      );
    });

    it("splits via regexes", () => {
      assert.deepStrictEqual(
        execute(parseOrThrow('split (regex "h.") @'),
          "ahi tuna"),
        [
          "a",
          " tuna"
        ]
      );
    });

    it("splits everywhere even with a non global regex", () => {
      assert.deepStrictEqual(
        execute(parseOrThrow('split (regex "a") @'),
          "babab"),
        [
          "b",
          "b",
          "b"
        ]
      );
    });
  });

  describe("#join", () => {
    it("joins basic arrays", () => {
      assert.deepStrictEqual(
        execute(parseOrThrow('join "" @'),
          [
            "bo",
            "ba"
          ]),
        "boba"
      );
    });

    it("joins with a delimiter", () => {
      assert.deepStrictEqual(
        execute(parseOrThrow('join ", " @'),
          [
            "bo",
            "ba"
          ]),
        "bo, ba"
      );
    });

    it("round-trips with split", () => {
      assert.deepStrictEqual(
        execute(parseOrThrow('(split ":" @ | join ":") == @'),
          "hello:hi:sup"),
        true
      );
    });
  });

  describe("#entries", () => {
    it("splits objects into entries", () => {
      assert.deepStrictEqual(
        execute(parseOrThrow("entries @"),
          {
            a: 1, b: 2
          }),
        [
          [
            "a",
            1
          ],
          [
            "b",
            2
          ],
        ]
      );
    });

    it("roundtrips with fromentries", () => {
      assert.deepStrictEqual(
        execute(parseOrThrow("(@ | entries | fromentries) == @"),
          {
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
        execute(parseOrThrow("fromentries @"),
          [
            [
              "a",
              1
            ],
            [
              "b",
              2
            ],
          ]),
        {
          a: 1, b: 2
        }
      );
    });

    it("roundtrips with entries", () => {
      assert.deepStrictEqual(
        execute(parseOrThrow("(@ | fromentries | entries) == @"),
          [
            [
              "a",
              1
            ],
            [
              "b",
              2
            ],
          ]),
        true
      );
    });

    it("uses nulls for missing values", () => {
      assert.deepStrictEqual(
        execute(parseOrThrow("fromentries @"),
          [
            [],
            [
              "b"
            ],
            [
              "c",
              1
            ]
          ]),
        {
          null: null,
          b: null,
          c: 1,
        }
      );
    });

    it("casts non-string keys to strings", () => {
      assert.deepStrictEqual(
        execute(parseOrThrow("fromentries @"),
          [
            [
              {},
              1
            ],
            [
              [
                1,
                2,
                3
              ],
              2
            ],
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
      assert.deepStrictEqual(execute(parseOrThrow("string @"),
        "hi"),
        "hi");
      assert.deepStrictEqual(execute(parseOrThrow("string @"),
        1.5),
        "1.5");
      assert.deepStrictEqual(execute(parseOrThrow("string @"), null),
        "null");
      assert.deepStrictEqual(
        execute(parseOrThrow("string @"),
          [
            1,
            2
          ]),
        "[1,2]"
      );
      assert.deepStrictEqual(execute(parseOrThrow("string @"), true),
        "true");
      assert.deepStrictEqual(execute(parseOrThrow("string @"), false),
        "false");
    });
  });

  describe("#float", () => {
    it("casts values to floats", () => {
      assert.deepStrictEqual(execute(parseOrThrow("float @"),
        "1.5"),
        1.5);
      assert.deepStrictEqual(
        execute(parseOrThrow("float @"),
          "10000.4"),
        10000.4
      );
      assert.deepStrictEqual(
        Number.isNaN(execute(parseOrThrow("float @"),
          "lol[]")),
        true
      );
      assert.throws(() => execute(parseOrThrow("float @"),
        []));
      assert.throws(() => execute(parseOrThrow("float @"),
        {}));
      assert.deepStrictEqual(execute(parseOrThrow("float @"), null),
        0);
      assert.deepStrictEqual(execute(parseOrThrow("float @"), true),
        1);
      assert.deepStrictEqual(execute(parseOrThrow("float @"), false),
        0);
    });
  });

  describe("#apply", () => {
    it("allows easy modification to a value via piping", () => {
      assert.deepStrictEqual(execute(parseOrThrow("@ | apply @ + 1"),
        1),
        2);
      assert.deepStrictEqual(
        execute(parseOrThrow("@ | apply @ + 1 | apply @ + 1"),
          1),
        3
      );
    });
    it("defines intermediate variables from the context variable", () => {
      assert.deepStrictEqual(
        execute(parseOrThrow("@ | apply a"),
          {
            a: 1, b: 2
          }),
        1
      );
      assert.deepStrictEqual(
        execute(parseOrThrow("@ | apply b"),
          {
            a: 1, b: 2
          }),
        2
      );
    });
  });
});
