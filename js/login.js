import { login as loginWasm } from "../wasm-client/pkg/wasm_client.js";
const USER_CREDENTIALS = "userCredentials";
export const loginFormEl = document.querySelector("#login-form");

export async function login(username, password, mode) {
  console.log("logging in");
  const userCredentials = await loginWasm(username, password, mode);
  userCredentials.mode = mode;
  localStorage.setItem(USER_CREDENTIALS, JSON.stringify(userCredentials));
  console.log({ userCredentials });
  return userCredentials;
}

export function fetchUserCredentials() {
  const userCredentialsString = localStorage.getItem(USER_CREDENTIALS);
  if (userCredentialsString) {
    const userCredentials = JSON.parse(userCredentialsString);
    console.log(`found credentials for mode: ${userCredentials.mode}`);
    console.log({ userCredentials });
    return userCredentials;
  } else if (loginFormEl.classList.contains("hidden")) {
    // Show login-form if credentials are not present
    console.log("No credentials. Show login.");
    loginFormEl.classList.remove("hidden");
  } else {
    console.log("No credentials present and login is already shown.");
  }
}
