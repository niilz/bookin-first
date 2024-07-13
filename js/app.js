import init, {
  login as loginWasm,
  courses as coursesWasm,
  slots as slotsWasm,
} from "../wasm-client/pkg/wasm_client.js";
import { mapCourseSlots } from "./course-mapper.js";
import { displayCourses, displaySlotsWeb } from "./display.js";
import { loadCourses } from "./course-action.js";

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
          console.log("Got courses. Hide login");
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
    console.log(`found credentials for mode: ${userCredentials.mode}`);
    console.log({ userCredentials });
    return await loadAndDisplayCourses(userCredentials, selectListEl);
  } else if (loginFormEl.classList.contains("hidden")) {
    // Show login-form if credentials are not present
    console.log("No credentials. Show login.");
    loginFormEl.classList.remove("hidden");
  } else {
    console.log("No credentials present and login is already shown.");
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
    await login(username, password, "web");
  }
  loadAndDisplayCourses(userCredentials, selectListEl);
});

coursesButtonAppMode.addEventListener("click", async (e) => {
  e.preventDefault();
  if (!userCredentials) {
    const username = usernameInputEl.value;
    const password = passwordInputEl.value;
    await login(username, password, "app");
  }
  loadAndDisplayCourses(userCredentials, selectListEl);
});

async function login(username, password, mode) {
  console.log("logging in");
  userCredentials = await loginWasm(username, password, mode);
  userCredentials.mode = mode;
  localStorage.setItem(USER_CREDENTIALS, JSON.stringify(userCredentials));
  console.log({ userCredentials });
}

async function loadAndDisplayCourses(userCredentials, selectListEl) {
  console.log("Loading courses");
  let courseResult = await loadCourses(userCredentials);
  const courseSlots = mapCourseSlots(courseResult, userCredentials.mode);
  displayCourses(courseSlots, selectListEl, userCredentials.mode);
  return courseResult;
}

// Only relevant for web mode (not app mode)
async function loadAndDisplaySlots(sessionId, courseId, selectListEl) {
  const slots = await slotsWasm(sessionId, courseId);
  displaySlotsWeb(slots, selectListEl);
}
