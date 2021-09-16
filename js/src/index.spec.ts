import assert from 'assert';
import { query } from ".";

describe("index", () => {
  describe('#query', () => {
    it("functions on a very basic level", () => {
      assert.deepStrictEqual(query("@ | map @ + 1", [1, 2, 3]), [2, 3, 4]);
    });
  });
});
