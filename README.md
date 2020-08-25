<div>
  <div align="center" style="display: block; text-align: center;">
    <img src="https://avatars3.githubusercontent.com/u/68873317?s=120&v=4" height="120" width="120" />
  </div>
  <h1 align="center">rust-lang-ve.github.io</h1>
  <span align="center">:octocat: rust-lang-ve's GitHub Page made with Yew because we 💖 Rust!</span>
</div>

## 🚴 Development

### Requirements

- GitHub Personal Access Token for GPM
- NodeJS (Yarn)
- Cargo

### Setting up .npmrc to fetch GPM

This project makes use of a package served by GPM (GitHub Package Manager).
In order to fetch this package you must create a `.npmrc` file in your home directory (`touch ~/.npmrc`), then
go to https://github.com/settings/tokens and clic on **Generate new token**, create your token with full access
to avoid issues fetching the package, copy your token to your clipboard and finally append the following to the `.npmrc` file you created before.

```bash
//npm.pkg.github.com/:_authToken=<YOUR PERSONAL ACCESS TOKEN>
```

Finally run `yarn` in the project directory to install packages.

> **IMPORTANT** Do not share your Personal Access Token with anyone! This token grant access to your GitHub profile!

### 🛠️ Build

When building for the first time, ensure to install dependencies first.

```
yarn install
```

```
yarn run build
```

### 🔬 Serve locally

```
yarn run start
```


## 🔋 Batteries Included

* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
* [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.
