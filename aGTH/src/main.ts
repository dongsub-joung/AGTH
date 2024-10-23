//main.ts
    
const wasmCode = await Deno.readFile("./target/wasm32-unknown-unknown/debug/wasm_deno_example.wasm");
const wasmModule = new WebAssembly.Module(wasmCode);
const wasmInstance = new WebAssembly.Instance(wasmModule);
const {
   text_body,
} = wasmInstance.exports;
console.log(text_body);
