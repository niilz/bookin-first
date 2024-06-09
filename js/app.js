import init, { login } from "../wasm-client/pkg/wasm_client.js";

async function run() {
  console.log("> run");
  await init();
  console.log("ran init");

  let loginRes = await login("User-Name", "Password");
  console.log({ loginRes });

  console.log({ res });
}

run();
