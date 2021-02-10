const {merge} = require("webpack-merge");
const base = require("./webpack.base.config");
/**
 * @type {import("webpack")}
 */
module.exports = merge(base, {
  mode: 'development',
});