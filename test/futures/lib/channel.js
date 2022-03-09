const assert = require("assert");

const { lazyAsyncAdd } = require("..");
const { assertRejects } = require("./util");

describe("Channel", () => {
  it("should be able to await channel result", async () => {
    const sum = await lazyAsyncAdd(
      () => 1,
      () => 2
    );

    assert.strictEqual(sum, 3);
  });

  it("exceptions should be handled", async () => {
    await assertRejects(async () => {
      await lazyAsyncAdd(
        () => 1,
        () => {
          throw new Error("Failed to get Y");
        }
      );
    }, /exception/i);
  });
});
