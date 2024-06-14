import init, { login, fetch_courses } from "../wasm-client/pkg/wasm_client.js";

async function initWasm() {
  console.log("> init");
  await init();
  console.log("< init");
}
initWasm()
  .then(() => console.log("initialized wasm module"))
  .catch((e) => console.error(`init failed ${e}`));

let userCredentials;

const usernameInput = document.querySelector("#username-input");
const passwordInput = document.querySelector("#password-input");

const loginButton = document.querySelector("#login-button");
loginButton.addEventListener("click", async (e) => {
  e.preventDefault();
  const username = usernameInput.value;
  const password = passwordInput.value;
  userCredentials = await login(username, password);

  console.log({ userCredentials });
});

const coursesButton = document.querySelector("#courses-button");
coursesButton.addEventListener("click", async (e) => {
  e.preventDefault();
  const { session } = userCredentials;
  const courses = await fetch_courses(session);

  console.log({ courses });
});
