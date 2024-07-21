import { login as loginWasm } from "../wasm-client/pkg/wasm_client.js";
const USER_CREDENTIALS = "userCredentials";
export const loginFormEl = document.querySelector("#login-form");

export async function login(username, password, mode) {
  if (username && password && mode) {
    console.log("logging in");
    try {
      const userCredentials = await loginWasm(username, password, mode);
      console.log("logged in");
      hideLoginForm();
      userCredentials.mode = mode;
      localStorage.setItem(USER_CREDENTIALS, JSON.stringify(userCredentials));
      //console.log({ userCredentials });
      return userCredentials;
    } catch (e) {
      console.warn(`Login failed. Error: ${e}`);
    }
  } else {
    console.warn("Login requires 'username', 'password' and 'mode'");
  }
}

export function fetchUserCredentials() {
  const userCredentialsString = localStorage.getItem(USER_CREDENTIALS);
  if (userCredentialsString) {
    const userCredentials = JSON.parse(userCredentialsString);
    console.log(`found credentials for mode: ${userCredentials.mode}`);
    //console.log({ userCredentials });
    return userCredentials;
  } else if (loginFormEl.classList.contains("hidden")) {
    // Show login-form if credentials are not present
    console.log("No credentials. Show login.");
    loginFormEl.classList.remove("hidden");
  } else {
    console.log("No credentials present and login is already visible.");
  }
}

export function clearUserCredentials() {
  window.localStorage.removeItem(USER_CREDENTIALS);
  console.log("Cleared user credentials");
}

export function hideLoginForm() {
  loginFormEl.classList.add("hidden");
  console.log("Hiding loginForm");
}

const clearCredentialsButton = document.querySelector(
  "#clear-credentials-button"
);
clearCredentialsButton.addEventListener("click", clearUserCredentials);
