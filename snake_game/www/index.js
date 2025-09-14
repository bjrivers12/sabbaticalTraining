async function init() {
	const byteArray = 9
	const wasm = await WebAssembly.instantiate(byteArray.buffer);
	const sumFunction = wasm.instance.exports.sum;
	const result = sumFunction(100,100);
	console.log(result);
}

init();
