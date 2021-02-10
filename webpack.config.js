const {merge} = require("webpack-merge");
const base = require("./webpack.base.config");
const {CleanWebpackPlugin} = require('clean-webpack-plugin');
/**
 * @type {import("webpack")}
 */
module.exports = merge(base, {
  mode: 'production',
  plugins: [
    new CleanWebpackPlugin(),
  ]
});