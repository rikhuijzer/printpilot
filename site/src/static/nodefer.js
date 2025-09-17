function writeToPtr(instance, ptr, text) {
    const buffer = instance.exports.memory.buffer;
    const view = new Uint8Array(buffer, ptr, 1024);
    const encoder = new TextEncoder();
    const with_stop = text + "<END>";
    view.set(encoder.encode(with_stop));
}

function readFromPtr(instance, ptr) {
    const buffer = instance.exports.memory.buffer;
    const view = new Uint8Array(buffer, ptr, 1024);
    const length = view.findIndex(byte => byte === 0);
    const decoder = new TextDecoder();

    return decoder.decode(new Uint8Array(buffer, ptr, length));
}

function pass(core) {
  const exports = core.instance.exports;
  const json = JSON.stringify({
    message: "Pass",
  });
  const ptr = exports.alloc();
  writeToPtr(core.instance, ptr, json);
  exports.hi(ptr);
  const result = readFromPtr(core.instance, ptr);
  exports.dealloc(ptr);
  console.log(`Received: ${result}`);
}

WebAssembly.instantiateStreaming(fetch("core.wasm")).then(
  (core) => {
    console.log("Loading of the PrintPilot WebAssembly library succeeded");
    pass(core);
  },
);
