import init from "/./assets/dioxus/keycrab_ext.js";
init("/./assets/dioxus/keycrab_ext_bg.wasm").then(wasm => {
  if (wasm.__wbindgen_start == undefined) {
    wasm.main();
  }
});
