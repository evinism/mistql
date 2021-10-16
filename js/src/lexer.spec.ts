import assert from "assert";
import { lex } from "./lexer";

describe("lexer", () => {
  describe("#lex", () => {
    it("should lex a basic expression", () => {
      assert.deepStrictEqual(lex("hello + there"), [
        { token: "ref", value: "hello", position: 0 },
        { token: "special", value: "+", position: 6 },
        { token: "ref", value: "there", position: 8 },
      ]);
    });

    it("should lex an expression with an initially ambiguous operator", () => {
      assert.deepStrictEqual(lex("hello || there"), [
        { token: "ref", value: "hello", position: 0 },
        { token: "special", value: "||", position: 6 },
        { token: "ref", value: "there", position: 9 },
      ]);
    });
    it("should lex a string", () => {
      assert.deepStrictEqual(lex('"sup" && there'), [
        { token: "value", value: "sup", position: 0 },
        { token: "special", value: "&&", position: 6 },
        { token: "ref", value: "there", position: 9 },
      ]);
    });

    it("handles escaped strings", () => {
      assert.deepStrictEqual(lex('"sup\\"" && there'), [
        { token: "value", value: 'sup"', position: 0 },
        { token: "special", value: "&&", position: 8 },
        { token: "ref", value: "there", position: 11 },
      ]);
      assert.deepStrictEqual(lex('"sup\\\\"   there'), [
        { token: "value", value: "sup\\", position: 0 },
        { token: "special", value: " ", position: 7 },
        { token: "ref", value: "there", position: 10 },
      ]);
      assert.deepStrictEqual(lex('"sup\\nnerd"   there'), [
        { token: "value", value: "sup\nnerd", position: 0 },
        { token: "special", value: " ", position: 11 },
        { token: "ref", value: "there", position: 14 },
      ]);

      assert.deepStrictEqual(lex('"sup\\tnerd"   there'), [
        { token: "value", value: "sup\tnerd", position: 0 },
        { token: "special", value: " ", position: 11 },
        { token: "ref", value: "there", position: 14 },
      ]);

      assert.deepStrictEqual(lex('"sup\\u0000 nerd"   there'), [
        { token: "value", value: "sup\u0000 nerd", position: 0 },
        { token: "special", value: " ", position: 16 },
        { token: "ref", value: "there", position: 19 },
      ]);
    });

    it("should error with unterminated strings", () => {
      assert.throws(() => lex('"sup'));
    });

    it("trims whitespace", () => {
      assert.deepStrictEqual(lex("  @  "), [
        { token: "ref", value: "@", position: 2 },
      ]);
    });

    it("parses any sequence of whitespace as a single space", () => {
      const cases = [
        "@ @",
        "@\t@",
        "@\n@",
        "@     @",
        "@\t  @",
        "@ \t @",
        "@  \t@",
        "@\n  @",
        "@ \n @",
        "@  \n@",
        "@  \n   \t  \t\t\t@",
      ];
      cases.forEach((item) =>
        assert.deepStrictEqual(lex(item), [
          { token: "ref", value: "@", position: 0 },
          { token: "special", value: " ", position: 1 },
          { token: "ref", value: "@", position: item.length - 1 },
        ])
      );
    });

    it("should handle right, left, and rl-binding tokens", () => {
      assert.deepStrictEqual(lex("  +  "), [
        { token: "special", value: "+", position: 2 },
      ]);

      ["(", "{", "["].forEach((token) => {
        assert.deepStrictEqual(lex(`@  ${token}  @`), [
          { token: "ref", value: "@", position: 0 },
          { token: "special", value: " ", position: 1 },
          { token: "special", value: token, position: 3 },
          { token: "ref", value: "@", position: 6 },
        ]);
      });

      [")", "}", "]"].forEach((token) => {
        assert.deepStrictEqual(lex(`@  ${token}  @`), [
          { token: "ref", value: "@", position: 0 },
          { token: "special", value: token, position: 3 },
          { token: "special", value: " ", position: 4 },
          { token: "ref", value: "@", position: 6 },
        ]);
      });
    });
  });
});
