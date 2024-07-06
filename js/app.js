import init, {
  login as loginWasm,
  courses as coursesWasm,
  slots as slotsWasm,
} from "../wasm-client/pkg/wasm_client.js";
import { displayCourses, displaySlots } from "./display.js";

const USER_CREDENTIALS = "userCredentials";

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
        // clear out stale-credentials
        localStorage.clear();
      });
  })
  .catch((e) => console.error(`init failed ${e}`));

let userCredentials;

// Login-Inputs
const loginFormEl = document.querySelector("#login-form");
const usernameInputEl = document.querySelector("#username-input");
const passwordInputEl = document.querySelector("#password-input");
// Displayed Data
const selectListEl = document.querySelector("#select-list");

async function tryLoadCourses() {
  const userCredentialsString = localStorage.getItem(USER_CREDENTIALS);
  if (userCredentialsString) {
    userCredentials = JSON.parse(userCredentialsString);
    console.log({ userCredentials });
    return await loadAndDisplayCourses(
      userCredentials.session,
      selectListEl,
      userCredentials.mode
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
    login(username, password, "web");
  }
  const { session } = userCredentials;
  loadAndDisplayCourses(session, "web");
});

coursesButtonAppMode.addEventListener("click", async (e) => {
  e.preventDefault();
  if (!userCredentials) {
    const username = usernameInputEl.value;
    const password = passwordInputEl.value;
    login(username, password, "app");
  }
  const { session } = userCredentials;
  loadAndDisplayCourses(session, "app");
});
async function login(username, password, mode) {
  userCredentials = await loginWasm(username, password, mode);
  userCredentials.mode = mode;
  localStorage.setItem(USER_CREDENTIALS, JSON.stringify(userCredentials));
  console.log({ userCredentials });
}

async function loadAndDisplayCourses(sessionId, selectListEl, mode) {
  const courseResult = await coursesWasm(sessionId, mode);
  displayCourses(courseResult, selectListEl);
  return courseResult;
}

// Only relevant for non web mode (not app mode)
async function loadAndDisplaySlots(sessionId, courseId, selectListEl) {
  const slots = await slotsWasm(sessionId, courseId);
  displaySlots(slots, selectListEl);
}
