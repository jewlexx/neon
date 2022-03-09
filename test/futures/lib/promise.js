const assert = require("assert");

const { lazyAsyncSum } = require("..");
const { assertRejects } = require("./util");

describe("JsFuture", () => {
  it("should be able to convert a promise to a future", async () => {
    const nums = new Float64Array([1, 2, 3, 4]);
    const sum = await lazyAsyncSum(async () => nums);

    assert.strictEqual(sum, 10);
  });

  it("should catch promise rejection", async () => {
    await assertRejects(async () => {
      await lazyAsyncSum(async () => {
        throw new Error("Oh, no!");
      });
    }, /exception/i);
  });
});
