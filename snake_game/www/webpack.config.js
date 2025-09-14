
const path = require("path");

module.export = {
	entry: "./index.js",
	output: {
	  path: path.resolve(__dirname, "public"),
		filename: "index.js"
	},
	mode: "development"
}
