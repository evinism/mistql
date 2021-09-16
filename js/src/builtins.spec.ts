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
        execute(parseOrThrow('map (blah + 1) arr'), { arr: [{ blah: 1 }, { blah: 2 }, { blah: 3 }] }), [2, 3, 4]);
    });
  });
});
