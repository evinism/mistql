import assert from "assert";
import { parseOrThrow } from "./parser";
import { ASTExpression } from "./types";

const lit = (type: any, value: any): ASTExpression => ({
  type: "literal",
  valueType: type,
  value,
});

const ref = (ref: string, internal?: true): ASTExpression => {
  const res: ASTExpression = {
    type: "reference",
    ref,
  };
  if (internal) {
    res.internal = true;
  }
  return res;
};

const par = (exp: ASTExpression): ASTExpression => ({
  type: "parenthetical",
  expression: exp,
});

const pipe = (stages: ASTExpression[]): ASTExpression => ({
  type: "pipeline",
  stages,
});

const app = (fn: ASTExpression, args: ASTExpression[] = []): ASTExpression => ({
  type: "application",
  function: fn,
  arguments: args,
});

describe("parser", () => {
  describe("#parse", () => {
    describe("overall", () => {
      it("fails to parse an empty statement", () => {
        assert.throws(() => parseOrThrow(""));
      });
    });

    describe("literals", () => {
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
          lit("object", {
            one: lit("number", 1),
            two: lit("number", 2),
            three: lit("number", 3),
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
                internal: true,
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
            internal: true,
          },
          type: "application",
        };

        assert.deepStrictEqual(parseOrThrow("@.hello.there"), target);
      });
    });

    describe("pipes", () => {
      it("parses a simple pipe", () => {
        assert.deepStrictEqual(
          parseOrThrow("hello |there"),
          pipe([ref("hello"), app(ref("there"))])
        );
      });

      it("parses a pipe with whitespace", () => {
        assert.deepStrictEqual(
          parseOrThrow("hello | there"),
          pipe([ref("hello"), app(ref("there"))])
        );
      });

      it("parses a pipe with a number of stages", () => {
        assert.deepStrictEqual(
          parseOrThrow("hello | there | hi | whatup"),
          pipe([
            ref("hello"),
            app(ref("there")),
            app(ref("hi")),
            app(ref("whatup")),
          ])
        );
      });
    });

    describe("parentheticals", () => {
      it("handles a basic parenthetical statement", () => {
        assert.deepStrictEqual(parseOrThrow("(hello)"), par(ref("hello")));
      });

      it("errors when parsing an empty parenthetical", () => {
        assert.throws(() => parseOrThrow("()"));
      });

      it("interops with pipes", () => {
        assert.deepStrictEqual(
          parseOrThrow("hello | (there) | hi | (whatup)"),
          pipe([
            ref("hello"),
            app(par(ref("there"))),
            app(ref("hi")),
            app(par(ref("whatup"))),
          ])
        );
      });

      it("handles a heavily nested parenthetical", () => {
        assert.deepStrictEqual(
          parseOrThrow("((((hello))))"),
          par(par(par(par(ref("hello")))))
        );
      });

      it("allows nested pipe expressions", () => {
        assert.deepStrictEqual(
          parseOrThrow("hello | (there | hi) | (whatup)"),
          pipe([
            ref("hello"),
            app(par(pipe([ref("there"), app(ref("hi"))]))),
            app(par(ref("whatup"))),
          ])
        );
      });
    });
    describe("applications", () => {
      it("parses a basic function application", () => {
        assert.deepStrictEqual(parseOrThrow("sup nernd hi"), {
          type: "application",
          _shouldntWrapInPipedExpressions: true,
          function: ref("sup"),
          arguments: [ref("nernd"), ref("hi")],
        });
      });

      it("parses function applications with parentheticals", () => {
        assert.deepStrictEqual(parseOrThrow("(sup) (nernd) (hi)"), {
          type: "application",
          _shouldntWrapInPipedExpressions: true,
          function: par(ref("sup")),
          arguments: [par(ref("nernd")), par(ref("hi"))],
        });
      });

      it("doesnt capture over pipes", () => {
        assert.deepStrictEqual(parseOrThrow("sup nernd | hi there"), {
          type: "pipeline",
          stages: [
            {
              type: "application",
              _shouldntWrapInPipedExpressions: true,
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
              _shouldntWrapInPipedExpressions: true,
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

    describe("dot accesses", () => {
      it("parses a deep series of items", () => {
        const target = {
          type: "application",
          function: {
            type: "reference",
            ref: ".",
            internal: true,
          },
          arguments: [
            {
              type: "application",
              function: {
                type: "reference",
                ref: ".",
                internal: true,
              },
              arguments: [
                {
                  type: "application",
                  function: {
                    type: "reference",
                    ref: ".",
                    internal: true,
                  },
                  arguments: [
                    {
                      type: "application",
                      function: {
                        type: "reference",
                        ref: ".",
                        internal: true,
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

      it("works after a parenthetical", () => {
        const target = {
          type: "application",
          function: {
            type: "reference",
            ref: ".",
            internal: true,
          },
          arguments: [
            {
              type: "application",
              function: {
                type: "reference",
                ref: ".",
                internal: true,
              },
              arguments: [
                par({
                  type: "application",
                  function: {
                    type: "reference",
                    ref: ".",
                    internal: true,
                  },
                  arguments: [
                    par({
                      type: "application",
                      function: {
                        type: "reference",
                        ref: ".",
                        internal: true,
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
                    }),
                    {
                      type: "reference",
                      ref: "much",
                    },
                  ],
                }),
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
        assert.deepStrictEqual(
          parseOrThrow("((there.is).much).to.learn"),
          target
        );
      });

      it("works after an object literal", () => {
        const target = {
          type: "application",
          function: {
            ref: ".",
            type: "reference",
            internal: true,
          },
          arguments: [
            {
              type: "literal",
              value: {
                hello: {
                  type: "literal",
                  value: 1,
                  valueType: "number",
                },
              },
              valueType: "object",
            },
            {
              ref: "hello",
              type: "reference",
            },
          ],
        };
        assert.deepStrictEqual(parseOrThrow("{hello: 1}.hello"), target);
      });
    });

    describe("indexing expressions", () => {
      it("handles a simple indexing case", () => {
        const expected = {
          type: "application",
          function: ref("index", true),
          arguments: [
            lit("number", 0),
            lit("array", [lit("number", 5), lit("number", 10)]),
          ],
        };
        assert.deepStrictEqual(parseOrThrow("[5, 10][0]"), expected);
      });

      it("handles indexing on objects", () => {
        const expected = {
          type: "application",
          function: ref("index", true),
          arguments: [
            lit("string", "hello"),
            lit("object", { hello: lit("string", "there") }),
          ],
        };
        assert.deepStrictEqual(
          parseOrThrow('{hello: "there"}["hello"]'),
          expected
        );
      });

      it("binds tighter than function application", () => {
        const expected = {
          type: "application",
          _shouldntWrapInPipedExpressions: true,
          function: ref("a"),
          arguments: [
            {
              type: "application",
              function: ref("index", true),
              arguments: [ref("c"), ref("b")],
            },
          ],
        };
        assert.deepStrictEqual(parseOrThrow("a b[c]"), expected);
      });

      it("works with parens", () => {
        const expected = {
          type: "application",
          function: ref("index", true),
          arguments: [
            ref("c"),
            par({
              type: "application",
              _shouldntWrapInPipedExpressions: true,
              function: ref("a"),
              arguments: [ref("b")],
            }),
          ],
        };
        assert.deepStrictEqual(parseOrThrow("(a b)[c]"), expected);
      });

      it("binds equivalently tightly to the dot accessor.", () => {
        const expected = {
          type: "application",
          function: ref(".", true),
          arguments: [
            {
              type: "application",
              function: ref("index", true),
              arguments: [
                ref("c"),
                {
                  type: "application",
                  function: ref(".", true),
                  arguments: [ref("a"), ref("b")],
                },
              ],
            },
            ref("d"),
          ],
        };
        assert.deepStrictEqual(parseOrThrow("a.b[c].d"), expected);
      });
    });

    describe("unary expressions", () => {
      it("parses basic binary expressions", () => {
        const expected = {
          type: "application",
          function: {
            type: "reference",
            ref: "-/unary",
            internal: true,
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
            internal: true,
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
                internal: true,
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
            internal: true,
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
            internal: true,
          },
          arguments: [
            {
              type: "application",
              function: {
                type: "reference",
                ref: "*",
                internal: true,
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
                internal: true,
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
            internal: true,
          },
          arguments: [
            {
              type: "application",
              function: {
                type: "reference",
                ref: "-",
                internal: true,
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
    });
    it("errors with incomplete binary expressions", () => {
      assert.throws(() => parseOrThrow("here +"));
      assert.throws(() => parseOrThrow("(here +)"));
      assert.throws(() => parseOrThrow("(1 * 2 *)"));
      assert.throws(() => parseOrThrow("a | b |"));
    });
  });
});
