import assert from "assert";
import { lex } from "./lexer";

describe("lexer", () => {
  describe("#lex", () => {
    it("should lex a basic expression", () => {
      assert.deepEqual(lex("hello + there"), [
        { token: "ref", value: "hello" },
        { token: "special", value: "+" },
        { token: "ref", value: "there" },
      ]);
    });

    it("should lex an expression with an initially ambiguous operator", () => {
      assert.deepEqual(lex("hello || there"), [
        { token: "ref", value: "hello" },
        { token: "special", value: "||" },
        { token: "ref", value: "there" },
      ]);
    });
    it("should lex a string", () => {
      assert.deepEqual(lex('"sup" && there'), [
        { token: "value", value: "sup" },
        { token: "special", value: "&&" },
        { token: "ref", value: "there" },
      ]);
    });

    it("handles escaped strings", () => {
      assert.deepEqual(lex('"sup\\"" && there'), [
        { token: "value", value: 'sup"' },
        { token: "special", value: "&&" },
        { token: "ref", value: "there" },
      ]);
      assert.deepEqual(lex('"sup\\\\"   there'), [
        { token: "value", value: "sup\\" },
        { token: "special", value: " " },
        { token: "ref", value: "there" },
      ]);
    });
  });
});
