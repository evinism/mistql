import assert from "assert";
import { execute } from "../executor";
import { parseOrThrow } from "../parser";

describe("builtin js specifics", () => {
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

  });

  describe("#float", () => {
    it("casts values to floats", () => {
      assert.throws(() => execute(parseOrThrow("float @"), "lol[]"));
      assert.throws(() => execute(parseOrThrow("float @"),
        []));
      assert.throws(() => execute(parseOrThrow("float @"),
        {}));
    });
  });
});
