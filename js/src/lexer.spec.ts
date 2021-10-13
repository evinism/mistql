import assert from "assert";
import { lex } from "./lexer";

describe("lexer", () => {
  describe("#lex", () => {
    it("should lex a basic expression", () => {
      assert.deepStrictEqual(lex("hello + there"), [
        { token: "ref", value: "hello" },
        { token: "special", value: "+" },
        { token: "ref", value: "there" },
      ]);
    });

    it("should lex an expression with an initially ambiguous operator", () => {
      assert.deepStrictEqual(lex("hello || there"), [
        { token: "ref", value: "hello" },
        { token: "special", value: "||" },
        { token: "ref", value: "there" },
      ]);
    });
    it("should lex a string", () => {
      assert.deepStrictEqual(lex('"sup" && there'), [
        { token: "value", value: "sup" },
        { token: "special", value: "&&" },
        { token: "ref", value: "there" },
      ]);
    });

    it("handles escaped strings", () => {
      assert.deepStrictEqual(lex('"sup\\"" && there'), [
        { token: "value", value: 'sup"' },
        { token: "special", value: "&&" },
        { token: "ref", value: "there" },
      ]);
      assert.deepStrictEqual(lex('"sup\\\\"   there'), [
        { token: "value", value: "sup\\" },
        { token: "special", value: " " },
        { token: "ref", value: "there" },
      ]);
    });

    it("should error with unterminated strings", () => {
      assert.throws(() => lex('"sup'));
    });

    it("trims whitespace", () => {
      assert.deepStrictEqual(lex('  @  '), [
        { token: "ref", value: "@" },
      ]);
    });

    it("parses any sequence of whitespace as a single space", () => {
      const cases = [
        '@ @',
        '@\t@',
        '@\n@',
        '@     @',
        '@\t  @',
        '@ \t @',
        '@  \t@',
        '@\n  @',
        '@ \n @',
        '@  \n@',
        '@  \n   \t  \t\t\t@',
      ];
      cases.forEach((item) => assert.deepStrictEqual(lex(item), [
        { token: "ref", value: '@' },
        { token: "special", value: " " },
        { token: "ref", value: '@' },
      ]));
    })

    it("should handle right, left, and rl-binding tokens", () => {
      assert.deepStrictEqual(lex("  +  "), [
        { token: "special", value: "+" },
      ]);

      ['(', '{', '['].forEach((token) => {
        assert.deepStrictEqual(lex(`@  ${token}  @`), [
          { token: "ref", value: '@' },
          { token: "special", value: " " },
          { token: "special", value: token },
          { token: "ref", value: '@' },

        ]);
      });

      [')', '}', ']'].forEach((token) => {
        assert.deepStrictEqual(lex(`@  ${token}  @`), [
          { token: "ref", value: '@' },
          { token: "special", value: token },
          { token: "special", value: " " },
          { token: "ref", value: '@' },
        ]);
      });
    });
  });
});
