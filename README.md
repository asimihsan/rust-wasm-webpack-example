<h1 align="center">
  rust-wasm-webpack-example
</h1>

<h4 align="center">Web application template for Rust, WebAssembly, and Webpack.</h4>

<p align="center">
  <a href="#usage">Usage</a> •
  <a href="#developer-notes">Developer notes</a> •
  <a href="#license">License</a>
</p>

`rust-wasm-webpack-example` is a template for building a web application using
Rust, WebAssembly, and Webpack. It takes the plain Rust code in the `rust`
crate, compiles it to WASM in `wasm-bindgen`, and bundles it with the JavaScript
code and Webpack build scripts in the `web` directory.

<p align="center">
  <img src="https://github.com/asimihsan/rust-wasm-webpack-example/actions/workflows/main.yaml/badge.svg" alt="GitHub Workflow Status" />
</p>

## Usage

Build production assets

```shell
make build
```

Development server

```shell
make serve
```

After playing around with this example code, you can start your own project.

- The `rust` directory contains a crate that is pure Rust. You would create one
  or more such crates yourself.
- The `rust-wasm-bindgen` directory contains a crate wraps the pure Rust code
  and compiles it to WASM using `wasm-bindgen`. You would create at least once
  such crate for your project. I would create one for each `rust` crate. I
  prefer to wrap the pure Rust code in a separate crate because it makes it
  clearer what code is pure Rust and what code is glue code needed for
  compilation to WASM.
- The `web` directory contains the JavaScript code and Webpack build scripts.
  You would create one such directory for your project.
  - [`web/webpack.common.js`](web/webpack.common.js) is the most important and
    complicated file. The `WasmPackPlugin` section is where the
    `rust-wasm-bindgen` crate is compiled to WASM.
  - Pay attention to the `src/pkg_*` subdirectories. These are the directories
    that contain the compiled WASM code as a resule of the Webpack config.

## Developer notes

Just notes for myself about how I started this project.

### Setting up `rust` from scratch

```
git init
cargo new rust
cargo new --lib rust-wasm-bindgen
(cd rust-wasm-bindgen && cargo add wasm-bindgen --features serde-serialize)
(cd rust-wasm-bindgen && cargo add console_error_panic_hook)
(cd rust-wasm-bindgen && cargo add serde --features derive)
(cd rust-wasm-bindgen && cargo add serde-derive)
```

### Setting up web from scratch

```

mkdir web
cd web
npm init -y
npm i --save-dev \
    @babel/core@7 \
    @babel/plugin-transform-runtime@7 \
    @babel/preset-env@7 \
    @babel/preset-typescript@7 \
    @typescript-eslint/eslint-plugin@4 \
    @typescript-eslint/parser@4 \
    @wasm-tool/wasm-pack-plugin@1 \
    acorn@8 \
    autoprefixer@10 \
    babel-loader@8 \
    copy-webpack-plugin@8 \
    css-loader@5 \
    css-minimizer-webpack-plugin@2 \
    eslint@7 \
    eslint-config-airbnb-base@15 \
    eslint-import-resolver-webpack@0 \
    eslint-plugin-import@2 \
    exports-loader@2 \
    html-loader@2 \
    html-webpack-plugin@5 \
    license-checker-rseidelsohn@2 \
    marked@4 \
    mini-css-extract-plugin@1 \
    node-sass@7 \
    postcss@8 \
    postcss-loader@5 \
    postcss-preset-env@7 \
    precss@3 \
    sass-loader@12 \
    style-loader@2 \
    tsutils@3 \
    typescript@4 \
    webpack@5 \
    webpack-cli@4 \
    webpack-dev-server@4 \
    webpack-merge@5
npm i \
    @popperjs/core@2 \
    bootstrap@5 \
    document-promises@4 \
    instant.page@5
npm audit
```

## License

This project is licensed under the terms of the Affero General Public License
v3.0. See the [LICENSE](LICENSE) file.
