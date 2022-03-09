const assert = require("assert");

async function assertRejects(f, ...args) {
  try {
    await f();
  } catch (err) {
    assert.throws(() => {
      throw err;
    }, ...args);

    return;
  }

  assert.throws(() => {}, ...args);
}

module.exports = {
  assertRejects,
};
