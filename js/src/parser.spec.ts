import assert from 'assert';
import { parseOrThrow } from './parser';
import { ASTExpression } from './types';

describe('#parse', () => {
  describe('literals', () => {
    const lit = (type: any, value: any): ASTExpression => ({
      type: 'literal',
      valueType: type,
      value,
    });

    it('parses numeric literals', () => {
      assert.deepStrictEqual(parseOrThrow('1'), lit('number', 1));
      assert.deepStrictEqual(parseOrThrow('2'), lit('number', 2));
      assert.deepStrictEqual(parseOrThrow('3'), lit('number', 3));
      assert.deepStrictEqual(parseOrThrow('349291'), lit('number', 349291));
    });

    it('parses the null literal', () => {
      assert.deepStrictEqual(parseOrThrow('null'), lit('null', null));
    });

    it('parses string literals', () => {
      assert.deepStrictEqual(parseOrThrow('"hi"'), lit('string', 'hi'));
      assert.deepStrictEqual(parseOrThrow('"there"'), lit('string', 'there'));
      assert.deepStrictEqual(parseOrThrow('"DOC OCK"'), lit('string', 'DOC OCK'));
    });

    it('parses array literals', () => {
      assert.deepStrictEqual(
        parseOrThrow('[1, 2, 3]'),
        lit('array', [lit('number', 1), lit('number', 2), lit('number', 3)]));
      assert.deepStrictEqual(
        parseOrThrow('["sup", "mr"]'),
        lit('array', [lit('string', 'sup'), lit('string', 'mr')]));
    });

    it('parses boolean literals', () => {
      assert.deepStrictEqual(parseOrThrow('true'), lit('boolean', true));
      assert.deepStrictEqual(parseOrThrow('false'), lit('boolean', false));
    });
  });

  describe('references', () => {
    it("parses bare references", () => {
      assert.deepStrictEqual(parseOrThrow('somefn'), { type: 'reference', path: ['somefn'] });
    });

    it("parses the root reference", () => {
      assert.deepStrictEqual(parseOrThrow('@'), { type: 'reference', path: ['@'] });
    });

    it("parses a path based on the root reference ", () => {
      assert.deepStrictEqual(parseOrThrow('@.hello.there'), { type: 'reference', path: ['@', 'hello', 'there'] });
    });

    it("parses a deep series of items", () => {
      assert.deepStrictEqual(
        parseOrThrow('there.is.much.to.learn'),
        { type: 'reference', path: ['there', 'is', 'much', 'to', 'learn'] });
    });
  });

  describe('pipelines', () => {
    it("parses a simple pipeline", () => {
      assert.deepStrictEqual(parseOrThrow('hello|there'), {
        type: 'pipeline', stages: [
          { type: 'reference', path: ['hello'] },
          { type: 'reference', path: ['there'] }
        ]
      });
    });

    it("parses a pipeline with whitespace", () => {
      assert.deepStrictEqual(parseOrThrow('hello | there'), {
        type: 'pipeline', stages: [
          { type: 'reference', path: ['hello'] },
          { type: 'reference', path: ['there'] }
        ]
      });
    });

    it("parses a pipeline with a number of stages", () => {
      assert.deepStrictEqual(parseOrThrow('hello | there | hi | whatup'), {
        type: 'pipeline', stages: [
          { type: 'reference', path: ['hello'] },
          { type: 'reference', path: ['there'] },
          { type: 'reference', path: ['hi'] },
          { type: 'reference', path: ['whatup'] }
        ]
      });
    });
  });
});