import assert from 'assert';
import { execute } from "./executor";
import { parseOrThrow } from "./parser";


describe("builtins", () => {
  describe('#map', () => {
    it("correctly maps simple values", () => {
      assert.deepEqual(execute(parseOrThrow('map @ + 1 [1, 2, 3]'), {}), [2, 3, 4]);
    });

    it("correctly maps structy values", () => {
      assert.deepEqual(
        execute(parseOrThrow('map (feature + 1) arr'), { arr: [{ feature: 1 }, { feature: 2 }, { feature: 3 }] }), [2, 3, 4]);
    });
  });

  describe('#filter', () => {
    it("correctly filters events", () => {
      assert.deepEqual(
        execute(parseOrThrow('filter type == "hi" events'),
          { events: [{ type: "hi", foo: 1 }, { type: "there", foo: 2 }] }
        ),
        [{ type: "hi", foo: 1 }]);
    });
  });

  describe('#keys', () => {
    it("correctly filters events", () => {
      assert.deepEqual(
        execute(parseOrThrow('@ | keys'),
          { type: "hi", foo: 1 }
        ),
        ["type", "foo"]);
    });
  });

  describe('#values', () => {
    it("correctly filters events", () => {
      assert.deepEqual(
        execute(parseOrThrow('@ | values'),
          { type: 5, foo: 1 }
        ),
        [5, 1]);
    });
  });

  describe('#groupby', () => {
    it("correctly groups events", () => {
      const events = [
        { type: "signup", email: "test1@example.com" },
        { type: "signup", email: "test2@example.com" },
        { type: "play", email: "test2@example.com" },
        { type: "play", email: "test2@example.com" },
      ];
      const expected = {
        'test1@example.com': [
          {
            email: 'test1@example.com',
            type: 'signup'
          }
        ],
        'test2@example.com': [
          {
            email: 'test2@example.com',
            type: 'signup'
          },
          {
            email: 'test2@example.com',
            type: 'play'
          },
          {
            email: 'test2@example.com',
            type: 'play'
          }
        ]
      }
      assert.deepEqual(
        execute(parseOrThrow('events | groupby email'),
          { events }
        ),
        expected);
    });
  });
});
