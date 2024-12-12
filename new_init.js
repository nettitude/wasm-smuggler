async function __wbg_init(input) {
  if (wasm !== undefined) return wasm;

  const imports = __wbg_get_imports();
  const base64Wasm = "{{BASE64_ENCODED_WASM}}";

  function base64ToArrayBuffer(base64) {
    var binary_string = window.atob(base64);
    var len = binary_string.length;
    var bytes = new Uint8Array(len);
    for (var i = 0; i < len; i++) {
      bytes[i] = binary_string.charCodeAt(i);
    }
    return bytes.buffer;
  }

  if (typeof input === "undefined") {
    input = base64ToArrayBuffer(base64Wasm);
  }
  __wbg_init_memory(imports);

  const { instance, module } = await __wbg_load(await input, imports);
  return __wbg_finalize_init(instance, module);
}
