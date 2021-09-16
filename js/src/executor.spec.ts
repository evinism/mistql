import assert from 'assert';
import { execute } from "./executor";
import { parseOrThrow } from "./parser";


describe("#execute", () => {
  describe('references', () => {
    it("handles a simple reference", () => {
      const result = execute(parseOrThrow('foo'), { foo: 1 });
      assert.strictEqual(result, 1);
    });

    it("executes deep references", () => {
      const result = execute(parseOrThrow('foo.bar.baz'), { foo: { bar: { baz: 1 } } });
      assert.strictEqual(result, 1);
    });

    it("executes more complicated deep references", () => {
      execute(parseOrThrow('foo.bar.baz'), { foo: { bar: { baz: 1 }, bleep: 2 } });
    });
  });

  describe('literals', () => {
    it("handles a simple string literal", () => {
      const result = execute(parseOrThrow('"foo"'), {});
      assert.strictEqual(result, 'foo');
    });

    it("handles a simple number literal", () => {
      const result = execute(parseOrThrow('58320'), {});
      assert.strictEqual(result, 58320);
    });

    it("handles a simple null literal", () => {
      const result = execute(parseOrThrow('null'), {});
      assert.strictEqual(result, null);
    });

    it("handles a simple array literal", () => {
      const result = execute(parseOrThrow('[1, 2]'), {});
      assert.deepStrictEqual(result, [1, 2]);
    });

    it("handles an array literal with references", () => {
      const result = execute(parseOrThrow('[foo.bar, baz]'), { foo: { bar: 5 }, baz: 6 });
      assert.deepStrictEqual(result, [5, 6]);
    });

    it("handles nested array literals", () => {
      const result = execute(parseOrThrow('[[foo.bar], baz]'), { foo: { bar: 5 }, baz: 6 });
      assert.deepStrictEqual(result, [[5], 6]);
    });
  });

  describe('pipe', () => {
    it("handles piping to parameterized functions", () => {
      const result = execute(parseOrThrow('foo | map @ + 1'), { foo: [1, 2, 3, 4, 5] });
      assert.deepStrictEqual(result, [2, 3, 4, 5, 6]);
    });
  });
})
