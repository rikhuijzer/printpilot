function pass() {
  const exports = core.instance.exports;
  const json = JSON.stringify({
    message: "Pass",
  });
  const ptr = exports.alloc(1024);
  writeToPtr(core.instance, ptr, json, json.length);
  // exports.hi(ptr);
  // const result = readFromPtr(core.instance, ptr, json.length);
  // exports.dealloc(ptr);
  // console.log(`Received: ${result}`);
}

var importObject = { imports: { imported_func: arg => console.log(arg) } };

WebAssembly.instantiateStreaming(fetch("core.wasm"), importObject).then(
  (core) => {
    window.core = core;
    console.log("Loading of the PrintPilot WebAssembly library succeeded");
    // pass();
  },
);
