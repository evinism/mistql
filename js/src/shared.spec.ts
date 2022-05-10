import assert from 'assert';
import { query } from '.';
import testdata from './shared/testdata.json';


const SELF_LANG_NAME = "js";

describe("Shared tests", () => {
  testdata.data.forEach((block) => {
    describe(block.describe, () => {
      block.cases.forEach((innerblock) => {
        describe(innerblock.describe, () => {
          innerblock.cases.forEach((testcase) => {
            const testCb = () => {
              testcase.assertions.forEach((assertion) => {
                if (assertion.throws) {
                  assert.throws(() => {
                    query(assertion.query, assertion.data);
                  });
                } else {
                  assert.deepStrictEqual(
                    query(assertion.query, assertion.data),
                    assertion.expected
                  );
                }
              });
            };
            if (testcase.skip && testcase.skip.includes(SELF_LANG_NAME)) {
              it.skip(testcase.it, testCb);
            } else {
              it(testcase.it, testCb);
            }
          });
        });
      });
    });
  });
});
