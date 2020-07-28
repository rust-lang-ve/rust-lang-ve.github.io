const path = require('path');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const CopyWebpackPlugin = require('copy-webpack-plugin');

const distPath = path.resolve(__dirname, "dist");
module.exports = (env, argv) => {
  return {
    devServer: {
      contentBase: distPath,
      compress: argv.mode === 'production',
      port: 8000
    },
    entry: './bootstrap.js',
    output: {
      path: distPath,
      filename: "website.js",
      webassemblyModuleFilename: "website.wasm"
    },
    module: {
      rules: [
        {
          test: /\.css$/i,
          use: [
            'style-loader',
            'css-loader',
          ],
        },
      ],
    },
    plugins: [
      new CopyWebpackPlugin([
        { from: './static', to: distPath }
      ]),
      new WasmPackPlugin({
        crateDirectory: ".",
        extraArgs: "--no-typescript",
      })
    ],
    watch: argv.mode !== 'production'
  };
};
