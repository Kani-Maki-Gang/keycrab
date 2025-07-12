import init, * as bindings from "./keycrab_ext.js";
const wasm = await init({
    module_or_path: "./dist/keycrab_ext_bg.wasm",
});

window.wasmBindings = bindings;

dispatchEvent(
    new CustomEvent("TrunkApplicationStarted", { detail: { wasm } })
);
