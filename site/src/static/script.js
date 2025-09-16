WebAssembly.instantiateStreaming(fetch("core.wasm")).then(
  (results) => {
    console.log("Loading of the PrintPilot WebAssembly library succeeded");
  },
);
