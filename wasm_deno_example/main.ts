//main.ts
    
const wasmCode = await Deno.readFile("./wasm_deno_example.wasm");
const wasmModule = new WebAssembly.Module(wasmCode);
const wasmInstance = new WebAssembly.Instance(wasmModule);
const {
   age,
} = wasmInstance.exports;
const yourAge = age(2020, 1994)
console.log(yourAge);
