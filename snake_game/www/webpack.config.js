
const path = require("path");
const CopyWebpackPlugin = require("copy-webpack-plugin");

module.exports = {
	mode: "development",
	entry: "./index.js",
	output: {
	  path: path.resolve(__dirname, "public"),
		filename: "bundle.js"
	},
	devServer: {
    static: path.resolve(__dirname, 'public'),
    port: 8080,
  },
	plugins: [
		new CopyWebpackPlugin({
			patterns: [
				{ from: "./index.html", to: "./"}
			]
		})
	]
}
