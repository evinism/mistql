import assert from 'assert';
import {seqHelper} from './util'

describe("util", () => {
  describe("#seqHelper", () => {
    it("handles the basic case of a single array", () => {
      assert.deepEqual(seqHelper(
        [[false, false, true, false, true]]
      ), [[2], [4]]);
    });

    it("handles getting the sequences of 2 arrays", () => {
      assert.deepEqual(seqHelper(
        [[false, true, true, false, true],
        [false, false, true, false, true]]
      ), [[1, 2], [1, 4], [2,4]]);
    });

    it("handles getting the sequences of 3 arrays", () => {
      assert.deepEqual(seqHelper(
        [[false, true, true, false, true, false, false],
        [false, false, true, false, true, false, false],
        [true, true, false, false, false, true, true]]
      ), [[1, 2, 5], [1, 2, 6], [1, 4, 5], [1, 4, 6], [2, 4, 5], [2, 4, 6]]);
    });
  });
});