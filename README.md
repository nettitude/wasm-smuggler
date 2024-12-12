# Next Level Smuggling with Web Assembly

1. Update `lib.rs` with username, password and payload (base64 encoded)

2. Run `wasm-pack build --target no-modules --release` to compile the library

3. Update the built `/pkg/*.js` file to replace the `__wbg_init` function with the code from `new_init.js`

4. Replace the `const base64Wasm = "{{BASE64_ENCODED_WASM}}";` in the new init function with the Base64 encoded `/pkg/*.wasm` file (you can use `base64 -w 0 *.wasm > base64wasm.txt` to get the Base64 encoded string)

5. Place the entire updated JavaScript code at the top of the `<script>` in the `template.html` file

6. Serve the `template.html` and try it out!
