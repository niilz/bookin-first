import init, { slots as slotsWasm } from "../wasm-client/pkg/wasm_client.js";
import { mapCourseSlots } from "./course-mapper.js";
import { displayCourses, displaySlotsWeb } from "./display.js";
import { loadCourses } from "./course-action.js";
import {
  clearUserCredentials,
  fetchUserCredentials,
  hideLoginForm,
  login,
  loginFormEl,
} from "./login.js";

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
          console.log("Trying to load courses on page load succeeded");
          hideLoginForm();
        } else {
          console.warn("Trying to load courses failed, please login first");
        }
      })
      .catch((e) => {
        console.error(`unexpected error during initial course loading: ${e}`);
        // show login form if courses could not be loaded
        loginFormEl.classList.remove("hidden");
        clearUserCredentials();
      });
  })
  .catch((e) => console.error(`init failed ${e}`));

// Login-Inputs
const usernameInputEl = document.querySelector("#username-input");
const passwordInputEl = document.querySelector("#password-input");
// Displayed Data
const selectListEl = document.querySelector("#select-list");

async function tryLoadCourses() {
  const userCredentials = fetchUserCredentials();
  if (userCredentials) {
    return loadAndDisplayCourses(userCredentials, selectListEl);
  }
}

selectListEl.addEventListener("click", async (e) => {
  const course = e.target;
  if (course.classList.contains("course")) {
    const userCredentials = fetchUserCredentials();
    loadAndDisplaySlots(
      userCredentials.session,
      course.dataset.courseId,
      selectListEl
    );
  }
});

const coursesButton = document.querySelector("#courses-button");
coursesButton.addEventListener("click", async (e) => {
  e.preventDefault();
  let userCredentials = fetchUserCredentials();
  if (!userCredentials) {
    const username = usernameInputEl.value;
    const password = passwordInputEl.value;
    userCredentials = await login(username, password, "web");
  }
  loadAndDisplayCourses(userCredentials, selectListEl);
});

const coursesButtonAppMode = document.querySelector("#courses-button-app-mode");
coursesButtonAppMode.addEventListener("click", async (e) => {
  e.preventDefault();
  let userCredentials = fetchUserCredentials();
  if (!userCredentials) {
    const username = usernameInputEl.value;
    const password = passwordInputEl.value;
    userCredentials = await login(username, password, "app");
  }
  loadAndDisplayCourses(userCredentials, selectListEl);
});

async function loadAndDisplayCourses(userCredentials, selectListEl) {
  if (!userCredentials) {
    console.warn("Cannot load coursed without user-credentials");
    return;
  }
  try {
    let courseResult = await loadCourses(userCredentials);
    const courseSlots = mapCourseSlots(courseResult, userCredentials.mode);
    displayCourses(courseSlots, selectListEl, userCredentials.mode);
    return courseResult;
  } catch (e) {
    console.log(`Could not load courses. Error: ${e}`);
    clearUserCredentials();
  }
}

// Only relevant for web mode (not app mode)
async function loadAndDisplaySlots(sessionId, courseId, selectListEl) {
  const slots = await slotsWasm(sessionId, courseId);
  displaySlotsWeb(slots, selectListEl);
}
