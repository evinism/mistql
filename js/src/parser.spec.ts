import assert from "assert";
import { parseOrThrow } from "./parser";
import { ASTExpression } from "./types";

describe("parser", () => {
  describe("#parse", () => {
    describe("overall", () => {
      it("fails to parse an empty statement", () => {
        assert.throws(() => parseOrThrow(""));
      });
    });

    describe("literals", () => {
      const lit = (type: any, value: any): ASTExpression => ({
        type: "literal",
        valueType: type,
        value,
      });

      it("parses numeric literals", () => {
        assert.deepStrictEqual(parseOrThrow("1"), lit("number", 1));
        assert.deepStrictEqual(parseOrThrow("2"), lit("number", 2));
        assert.deepStrictEqual(parseOrThrow("3"), lit("number", 3));
        assert.deepStrictEqual(parseOrThrow("349291"), lit("number", 349291));
        assert.deepStrictEqual(parseOrThrow("0.05"), lit("number", 0.05));
      });

      it("parses the null literal", () => {
        assert.deepStrictEqual(parseOrThrow("null"), lit("null", null));
      });

      it("parses string literals", () => {
        assert.deepStrictEqual(parseOrThrow('"hi"'), lit("string", "hi"));
        assert.deepStrictEqual(parseOrThrow('"there"'), lit("string", "there"));
        assert.deepStrictEqual(
          parseOrThrow('"DOC OCK"'),
          lit("string", "DOC OCK")
        );
      });

      it("parses array literals", () => {
        assert.deepStrictEqual(
          parseOrThrow("[1, 2, 3]"),
          lit("array", [lit("number", 1), lit("number", 2), lit("number", 3)])
        );
        assert.deepStrictEqual(
          parseOrThrow('["sup", "mr"]'),
          lit("array", [lit("string", "sup"), lit("string", "mr")])
        );
      });

      it("parses boolean literals", () => {
        assert.deepStrictEqual(parseOrThrow("true"), lit("boolean", true));
        assert.deepStrictEqual(parseOrThrow("false"), lit("boolean", false));
      });


      it("parses object literals", () => {
        assert.deepStrictEqual(
          parseOrThrow("{one: 1, two: 2, three: 3}"),
          lit("struct", {
            one: lit("number", 1),
            two: lit("number", 2),
            three: lit("number", 3)
          })
        );
      });

    });

    describe("references", () => {
      it("parses bare references", () => {
        assert.deepStrictEqual(parseOrThrow("somefn"), {
          type: "reference",
          ref: "somefn",
        });
      });

      it("parses the root reference", () => {
        assert.deepStrictEqual(parseOrThrow("@"), {
          type: "reference",
          ref: "@",
        });
      });

      it("parses a path based on the root reference ", () => {
        const target = {
          arguments: [
            {
              arguments: [
                {
                  ref: "@",
                  type: "reference",
                },
                {
                  ref: "hello",
                  type: "reference",
                },
              ],
              function: {
                ref: ".",
                type: "reference",
              },
              type: "application",
            },
            {
              ref: "there",
              type: "reference",
            },
          ],
          function: {
            ref: ".",
            type: "reference",
          },
          type: "application",
        };

        assert.deepStrictEqual(parseOrThrow("@.hello.there"), target);
      });

      it("parses a deep series of items", () => {
        const target = {
          type: "application",
          function: {
            type: "reference",
            ref: ".",
          },
          arguments: [
            {
              type: "application",
              function: {
                type: "reference",
                ref: ".",
              },
              arguments: [
                {
                  type: "application",
                  function: {
                    type: "reference",
                    ref: ".",
                  },
                  arguments: [
                    {
                      type: "application",
                      function: {
                        type: "reference",
                        ref: ".",
                      },
                      arguments: [
                        {
                          type: "reference",
                          ref: "there",
                        },
                        {
                          type: "reference",
                          ref: "is",
                        },
                      ],
                    },
                    {
                      type: "reference",
                      ref: "much",
                    },
                  ],
                },
                {
                  type: "reference",
                  ref: "to",
                },
              ],
            },
            {
              type: "reference",
              ref: "learn",
            },
          ],
        };
        assert.deepStrictEqual(parseOrThrow("there.is.much.to.learn"), target);
      });
    });

    describe("pipes", () => {
      it("parses a simple pipe", () => {
        assert.deepStrictEqual(parseOrThrow("hello|there"), {
          type: "pipeline",
          stages: [
            { type: "reference", ref: "hello" },
            { type: "reference", ref: "there" },
          ],
        });
      });

      it("parses a pipe with whitespace", () => {
        assert.deepStrictEqual(parseOrThrow("hello | there"), {
          type: "pipeline",
          stages: [
            { type: "reference", ref: "hello" },
            { type: "reference", ref: "there" },
          ],
        });
      });

      it("parses a pipe with a number of stages", () => {
        assert.deepStrictEqual(parseOrThrow("hello | there | hi | whatup"), {
          type: "pipeline",
          stages: [
            { type: "reference", ref: "hello" },
            { type: "reference", ref: "there" },
            { type: "reference", ref: "hi" },
            { type: "reference", ref: "whatup" },
          ],
        });
      });
    });

    describe("parentheticals", () => {
      it("handles a basic parenthetical statement", () => {
        assert.deepStrictEqual(parseOrThrow("(hello)"), {
          type: "reference",
          ref: "hello",
        });
      });

      it("errors when parsing an empty parenthetical", () => {
        assert.throws(() => parseOrThrow("()"));
      });

      it("interops with pipes", () => {
        assert.deepStrictEqual(
          parseOrThrow("hello | (there) | hi | (whatup)"),
          {
            type: "pipeline",
            stages: [
              { type: "reference", ref: "hello" },
              { type: "reference", ref: "there" },
              { type: "reference", ref: "hi" },
              { type: "reference", ref: "whatup" },
            ],
          }
        );
      });

      it("handles a heavily nested parenthetical", () => {
        assert.deepStrictEqual(parseOrThrow("((((hello))))"), {
          type: "reference",
          ref: "hello",
        });
      });

      it("allows nested pipe expressions", () => {
        assert.deepStrictEqual(
          parseOrThrow("hello | (there | hi) | (whatup)"),
          {
            type: "pipeline",
            stages: [
              { type: "reference", ref: "hello" },
              {
                type: "pipeline",
                stages: [
                  { type: "reference", ref: "there" },
                  { type: "reference", ref: "hi" },
                ],
              },
              { type: "reference", ref: "whatup" },
            ],
          }
        );
      });
    });
    describe("applications", () => {
      it("parses a basic function application", () => {
        assert.deepStrictEqual(parseOrThrow("sup nernd hi"), {
          type: "application",
          function: {
            type: "reference",
            ref: "sup",
          },
          arguments: [
            {
              type: "reference",
              ref: "nernd",
            },
            {
              type: "reference",
              ref: "hi",
            },
          ],
        });
      });

      it("parses function applications with parentheticals", () => {
        assert.deepStrictEqual(parseOrThrow("(sup) (nernd) (hi)"), {
          type: "application",
          function: {
            type: "reference",
            ref: "sup",
          },
          arguments: [
            {
              type: "reference",
              ref: "nernd",
            },
            {
              type: "reference",
              ref: "hi",
            },
          ],
        });
      });

      it("doesnt capture over pipes", () => {
        assert.deepStrictEqual(parseOrThrow("sup nernd | hi there"), {
          type: "pipeline",
          stages: [
            {
              type: "application",
              function: {
                type: "reference",
                ref: "sup",
              },
              arguments: [
                {
                  type: "reference",
                  ref: "nernd",
                },
              ],
            },
            {
              type: "application",
              function: {
                type: "reference",
                ref: "hi",
              },
              arguments: [
                {
                  type: "reference",
                  ref: "there",
                },
              ],
            },
          ],
        });
      });
    });

    describe("unary expressions", () => {
      it("parses basic binary expressions", () => {
        const expected = {
          type: "application",
          function: {
            type: "reference",
            ref: "-/unary",
          },
          arguments: [
            {
              type: "reference",
              ref: "here",
            },
          ],
        };
        assert.deepStrictEqual(parseOrThrow("-here"), expected);
      });

      it("parses mixing binary and unary expressions", () => {
        const expected = {
          type: "application",
          function: {
            type: "reference",
            ref: "+",
          },
          arguments: [
            {
              type: "reference",
              ref: "there",
            },
            {
              type: "application",
              function: {
                type: "reference",
                ref: "-/unary",
              },
              arguments: [
                {
                  type: "reference",
                  ref: "here",
                },
              ],
            },
          ],
        };
        assert.deepStrictEqual(parseOrThrow("there + -here"), expected);
      });
    });

    describe("binary expressions", () => {
      it("parses basic binary expressions", () => {
        const expected = {
          type: "application",
          function: {
            type: "reference",
            ref: "+",
          },
          arguments: [
            {
              type: "reference",
              ref: "here",
            },
            {
              type: "reference",
              ref: "there",
            },
          ],
        };
        assert.deepStrictEqual(parseOrThrow("here + there"), expected);
      });

      it("respects operator precedence", () => {
        const expected = {
          type: "application",
          function: {
            type: "reference",
            ref: "+",
          },
          arguments: [
            {
              type: "application",
              function: {
                type: "reference",
                ref: "*",
              },
              arguments: [
                {
                  type: "reference",
                  ref: "one",
                },
                {
                  type: "reference",
                  ref: "two",
                },
              ],
            },
            {
              type: "application",
              function: {
                type: "reference",
                ref: "*",
              },
              arguments: [
                {
                  type: "reference",
                  ref: "three",
                },
                {
                  type: "reference",
                  ref: "four",
                },
              ],
            },
          ],
        };
        assert.deepStrictEqual(
          parseOrThrow("one * two + three * four"),
          expected
        );
      });

      it("respects left associativity", () => {
        const expected = {
          type: "application",
          function: {
            type: "reference",
            ref: "-",
          },
          arguments: [
            {
              type: "application",
              function: {
                type: "reference",
                ref: "-",
              },
              arguments: [
                {
                  type: "reference",
                  ref: "one",
                },
                {
                  type: "reference",
                  ref: "two",
                },
              ],
            },
            {
              type: "reference",
              ref: "three",
            },
          ],
        };
        assert.deepStrictEqual(parseOrThrow("one - two - three"), expected);
      });
      it("respects all these associativity comparisons", () => {
        const comparisons: [string, string][] = [
          ["one - two - three", "(one - two) - three"],
          ["one - two * three", "one - (two * three)"],
          ["a == b * 5", "a == (b * 5)"],
          ["a / 3 + 2 == b * 5", "((a / 3) + 2) == (b * 5)"],
          ["a / 3 + 2 == b * 5 d | c", "(((a / 3) + 2) == (b * 5)) d | c"],
        ];
        comparisons.forEach(([l, r]) => {
          assert.deepStrictEqual(parseOrThrow(l), parseOrThrow(r));
        });
      });
    });
  });
});
