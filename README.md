# lazor
A (naive) ray tracer for WebAssembly. Most of the code was stolen from [jsoendermann/ljsp](https://github.com/jsoendermann/ljsp).

# Setup
Install Rust (e.g. via [Rustup](https://www.rust-lang.org/tools/install)),
[wasm-pack](https://rustwasm.github.io/wasm-pack/installer/), and [Node.js](https://nodejs.org/en/):
```bash
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
$ curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
$ curl -O https://nodejs.org/dist/v12.16.1/node-v12.16.1-linux-x64.tar.xz && tar xf node*
```

Install the Node packages:
```bash
$ cd www/
$ npm install
$ npm install wasm-opt -g
```

> Installing wasm-opt globally fixes a current bug!

# How to build it?
Build via wasm-pack and run the NPM sever:
```bash
$ wasm-pack build
$ cd www/
$ npm run start
```
