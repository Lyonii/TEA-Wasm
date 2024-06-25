import("@tea/tea-wasm").then(async ({ base64_encrypt, base64_decrypt, tea_encrypt, tea_decrypt }) => {
  const e = tea_encrypt("Hello Word!", 'Pr4nOjvzNc2BMHpMK/e4Aw==')
  console.log(e);
  const d = tea_decrypt(base64_decrypt('kS9uGmOCs6ZyZCE='), 'Pr4nOjvzNc2BMHpMK/e4Aw==')
  const td = new TextDecoder()
  console.log(td.decode(d))
  const te = new TextEncoder()
  console.log(te.encode('21313123asd@@@@#'))
})
