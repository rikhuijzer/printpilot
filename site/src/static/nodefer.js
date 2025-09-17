function writeToPtr(instance, ptr, text, bufferLength) {
    const buffer = instance.exports.memory.buffer;
    const view = new Uint8Array(buffer, ptr, bufferLength);
    const encoder = new TextEncoder();
    const with_stop = text + "<END>";
    view.set(encoder.encode(with_stop));
}

function readFromPtr(instance, ptr, bufferLength) {
    const buffer = instance.exports.memory.buffer;
    const view = new Uint8Array(buffer, ptr, bufferLength);
    const length = view.findIndex(byte => byte === 0);
    const decoder = new TextDecoder();

    return decoder.decode(new Uint8Array(buffer, ptr, length));
}

function pass() {
  const exports = core.instance.exports;
  const json = JSON.stringify({
    message: "Pass",
  });
  const ptr = exports.alloc(1024);
  writeToPtr(core.instance, ptr, json, 1024);
  exports.hi(ptr);
  const result = readFromPtr(core.instance, ptr, 1024);
  exports.dealloc(ptr);
  console.log(`Received: ${result}`);
}

WebAssembly.instantiateStreaming(fetch("core.wasm")).then(
  (core) => {
    window.core = core;
    console.log("Loading of the PrintPilot WebAssembly library succeeded");
    pass();
  },
);
