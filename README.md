This reproduces the memory leak that occurs when using `tsify`'s `wasm-bindgen` integration.

```
tsify-leak-repro $ ./repro.sh
+ set -o errexit
+ cargo bin wasm-pack build --target nodejs
[INFO]: 🎯  Checking for the Wasm target...
[INFO]: 🌀  Compiling to Wasm...
   Compiling tsify-macros v0.5.5 (/Users/me/workspace/github.com/utdemir/tsify/tsify-macros)
   Compiling tsify v0.5.5 (/Users/me/workspace/github.com/utdemir/tsify)
   Compiling tsify-leak-repro v0.1.0 (/Users/me/workspace/github.com/utdemir/tsify-leak-repro)
    Finished `release` profile [optimized] target(s) in 0.94s
[INFO]: ⬇️  Installing wasm-bindgen...
[INFO]: Optimizing wasm binaries with `wasm-opt`...
[INFO]: Optional fields missing from Cargo.toml: 'description', 'repository', and 'license'. These are not necessary, but recommended
[INFO]: ✨   Done in 1.41s
[INFO]: 📦   Your wasm pkg is ready to publish at /Users/me/workspace/github.com/utdemir/tsify-leak-repro/pkg.
+ node index.js
/Users/me/workspace/github.com/utdemir/tsify-leak-repro/index.js:12
      throw err;
      ^

RuntimeError: memory access out of bounds
    at wasm://wasm/0003e432:wasm-function[32]:0x4b2f
    at wasm://wasm/0003e432:wasm-function[42]:0x5f70
    at wasm://wasm/0003e432:wasm-function[59]:0x7406
    at wasm://wasm/0003e432:wasm-function[29]:0x4490
    at wasm://wasm/0003e432:wasm-function[87]:0xbec7
    at module.exports.get_bar_via_tsify (/Users/me/workspace/github.com/utdemir/tsify-leak-repro/pkg/tsify_leak_repro.js:172:26)
    at Object.<anonymous> (/Users/me/workspace/github.com/utdemir/tsify-leak-repro/index.js:6:5)
    at Module._compile (node:internal/modules/cjs/loader:1546:14)
    at Object..js (node:internal/modules/cjs/loader:1698:10)
    at Module.load (node:internal/modules/cjs/loader:1303:32)

Node.js v23.3.0
```