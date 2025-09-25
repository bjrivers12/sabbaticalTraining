const path = require("path");
const CopyWebpackPlugin = require("copy-webpack-plugin");
const webpack = require("webpack");

module.exports = {
  mode: "development",
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "public"),
    filename: "bundle.js",
  },
  devServer: {
    static: path.resolve(__dirname, "public"),
    port: 8080,
  },
  experiments: {
    asyncWebAssembly: true,
  },
  resolve: {
    fallback: {
      stream: require.resolve("stream-browserify"),
      buffer: require.resolve("buffer/"),
      process: require.resolve("process/browser"),
    },
  },

  plugins: [
    new CopyWebpackPlugin({
      patterns: [{ from: "./index.html", to: "./" }],
    }),
    new webpack.ProvidePlugin({
      Buffer: ["buffer", "Buffer"],
      process: "process/browser",
    }),
  ],
};
