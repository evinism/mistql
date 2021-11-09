import assert from 'assert';
import { query } from '.';
import testdata from './shared/testdata.json';

describe("Shared tests", () => {
  testdata.data.forEach((block) => {
    describe(block.describe, () => {
      block.cases.forEach((innerblock) => {
        describe(innerblock.describe, () => {
          innerblock.cases.forEach((testcase) => {
            it(testcase.it, () => {
              testcase.assertions.forEach(assertion => {
                assert.deepStrictEqual(query(assertion.query, assertion.data), assertion.expected);
              })
            })
          });
        });
      });
    })
  });
});
