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
    entry: './launch.js',
    output: {
      path: distPath,
      filename: "mywasm.js",
      webassemblyModuleFilename: "mywasm.wasm"
    },
    plugins: [
      new CopyWebpackPlugin({
        patterns: [
          { from: './public', to: distPath },
        ],
      }),
      new WasmPackPlugin({
        crateDirectory: ".",
        extraArgs: "--no-typescript",
      })
    ],
    experiments: {
      syncWebAssembly: true,
    },
    watch: argv.mode !== 'production'
  };
};