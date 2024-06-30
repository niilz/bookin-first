import init, {
  login as loginWasm,
  courses as coursesWasm,
  slots as slotsWasm,
} from "../wasm-client/pkg/wasm_client.js";
import { displayCourses, displaySlots } from "./display.js";

const USER_CREDENTIALS = "userCredentials";
const USER_CREDENTIALS_APP_MODE = "userCredentialsAppMode";

async function initWasm() {
  console.log("> init");
  await init();
  console.log("< init");
}
initWasm()
  .then(() => {
    console.log("initialized wasm module");
    tryLoadCourses()
      .then((courseResult) => {
        if (courseResult) {
          // hide login form if courses could be loaded
          loginFormEl.classList.add("hidden");
        }
      })
      .catch((e) => {
        console.error(`unexpected error during initial course loading: ${e}`);
        // show login form if courses could not be loaded
        loginFormEl.classList.remove("hidden");
      });
  })
  .catch((e) => console.error(`init failed ${e}`));

let userCredentials;
let userCredentialsAppMode;

// Login-Inputs
const loginFormEl = document.querySelector("#login-form");
const usernameInputEl = document.querySelector("#username-input");
const passwordInputEl = document.querySelector("#password-input");
// Displayed Data
const selectListEl = document.querySelector("#select-list");

async function tryLoadCourses() {
  const userCredentialsString = localStorage.getItem(USER_CREDENTIALS);
  const userCredentialsAppModeString = localStorage.getItem(
    USER_CREDENTIALS_APP_MODE
  );
  if (userCredentialsString) {
    userCredentials = JSON.parse(userCredentialsString);
    console.log({ userCredentials });
    return await loadAndDisplayCourses(userCredentials.session, selectListEl);
  } else if (userCredentialsAppModeString) {
    userCredentialsAppMode = JSON.parse(userCredentialsAppMode);
    console.log({ userCredentialsAppMode });
    return await loadAndDisplayCourses(
      userCredentials.session,
      selectListEl,
      true
    );
  } else if (loginFormEl.classList.contains("hidden")) {
    // Show login-form if credentials are not present
    loginFormEl.classList.remove("hidden");
  }
}

selectListEl.addEventListener("click", async (e) => {
  const course = e.target;
  if (course.classList.contains("course")) {
    loadAndDisplaySlots(
      userCredentials.session,
      course.dataset.courseId,
      selectListEl
    );
  }
});

const coursesButton = document.querySelector("#courses-button");
const coursesButtonAppMode = document.querySelector("#courses-button-app-mode");
coursesButton.addEventListener("click", async (e) => {
  e.preventDefault();
  if (!userCredentials) {
    const username = usernameInputEl.value;
    const password = passwordInputEl.value;
    login(username, password);
  }
  const { session } = userCredentials;
  loadAndDisplayCourses(session);
});

coursesButtonAppMode.addEventListener("click", async (e) => {
  e.preventDefault();
  if (!userCredentialsAppMode) {
    const username = usernameInputEl.value;
    const password = passwordInputEl.value;
    login(username, password, true);
  }
  const { session } = userCredentials;
  loadAndDisplayCourses(session, true);
});
async function login(username, password, isAppMode) {
  if (isAppMode) {
    userCredentialsAppMode = await loginWasmAppMode(username, password);
    localStorage.setItem(
      USER_CREDENTIALS_APP_MODE,
      JSON.stringify(userCredentialsAppMode)
    );
    console.log({ userCredentialsAppMode });
  } else {
    userCredentials = await loginWasm(username, password);
    localStorage.setItem(USER_CREDENTIALS, JSON.stringify(userCredentials));
    console.log({ userCredentials });
  }
}

async function loadAndDisplayCourses(sessionId, isAppMode) {
  let courseResult;
  if (isAppMode) {
    courseResult = await coursesWasmAppMode(sessionId);
  } else {
    courseResult = await coursesWasm(sessionId);
  }
  displayCourses(courseResult, selectListEl);
  return courseResult;
}

// Only relevant for non web mode (not app mode)
async function loadAndDisplaySlots(sessionId, courseId, selectListEl) {
  const slots = await slotsWasm(sessionId, courseId);
  displaySlots(slots, selectListEl);
}
