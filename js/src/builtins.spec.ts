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
});
