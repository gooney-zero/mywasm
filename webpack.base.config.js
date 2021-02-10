const path = require('path');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const CssMinimizerPlugin = require("css-minimizer-webpack-plugin");
const HtmlWebpackPlugin = require('html-webpack-plugin');
const TerserPlugin = require("terser-webpack-plugin");
const MiniCssExtractPlugin = require('mini-css-extract-plugin');

const distPath = path.resolve(__dirname, "dist");
/**
 * @type {import("webpack")}
 */
module.exports = {
  // devtool: process.env.NODE_ENV === 'production' ? 'source-map' : 'inline-source-map', // 官方推荐生产环境和开发环境的配置
  entry: './launch.js',
  output: {
    path: distPath,
    filename: "mywasm.js",
    webassemblyModuleFilename: "mywasm.wasm"
  },
  experiments: {
    syncWebAssembly: true,
  },
  module: {
    rules: [
      {
        test: /\.css/,
        use: [MiniCssExtractPlugin.loader, "css-loader", "postcss-loader"],
      },
    ]
  },
  performance: {
    hints: "warning", // 枚举
    hints: "error", // 性能提示中抛出错误
    hints: false, // 关闭性能提示
    maxAssetSize: 200000, // 整数类型（以字节为单位）
    maxEntrypointSize: 400000, // 整数类型（以字节为单位）
    assetFilter: function (assetFilename) {
      // 提供资源文件名的断言函数
      return assetFilename.endsWith('.css') || assetFilename.endsWith('.js');
    }
  },
  plugins: [

    new WasmPackPlugin({
      crateDirectory: ".",
      extraArgs: "--no-typescript",
    }),

    new MiniCssExtractPlugin({
      filename: "[hash].style.css"
    }),

    new HtmlWebpackPlugin({
      template: "./public/index.html",
    })
  ],
  optimization: {
    minimizer: [
      new CssMinimizerPlugin({
        parallel: true, // 可省略，默认开启并行
        sourceMap: true, // 可省略，默认遵循webpack的devtool配置
        minimizerOptions: {
          preset: 'advanced', // 需额外安装
        },
      }),
      new TerserPlugin({
        parallel: true, // 可省略，默认开启并行
        terserOptions: {
          toplevel: true, // 最高级别，删除无用代码
          ie8: true,
          safari10: true,
        }
      })

    ]
  }
};