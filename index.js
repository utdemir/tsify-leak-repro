const { get_bar_via_tsify, get_bar_without_tsify } = require('./pkg/tsify_leak_repro.js');


for (let j = 0; j < 100_000; j++) {
  try {
    get_bar_via_tsify({ "notbar": "baz" });
    // get_bar_without_tsify({ "notbar": "baz" });
  } catch (err) {
    if (err.message.includes('missing field')) {
      // Expected error, continue
    } else {
      throw err;
    }
  }
}

console.log('Done');