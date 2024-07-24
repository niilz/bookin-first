import init, { slots as slotsWasm } from "../wasm-client/pkg/wasm_client.js";
import { displayCourses, displaySlotsWeb } from "./display.js";
import { getCourseData, loadCourses, setCourseData } from "./course-action.js";
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

async function tryLoadCourses() {
  const userCredentials = fetchUserCredentials();
  return loadAndDisplayCourses(userCredentials);
}

const coursesButton = document.querySelector("#courses-button");
coursesButton.addEventListener("click", async (e) => {
  e.preventDefault();
  let userCredentials = fetchUserCredentials();
  if (!userCredentials) {
    const username = usernameInputEl.value;
    const password = passwordInputEl.value;
    userCredentials = await login(username, password, "web");
  }
  loadAndDisplayCourses(userCredentials);
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
  loadAndDisplayCourses(userCredentials);
});

async function loadAndDisplayCourses(userCredentials) {
  const mode = userCredentials ? userCredentials.mode : "app";
  try {
    let courseSlots = await loadCourses(userCredentials, mode);
    if (!courseSlots) {
      console.log("Could not load courses");
      return;
    }
    setCourseData(courseSlots);
    displayCourses(getCourseData(), mode);
    return courseSlots;
  } catch (e) {
    console.log(`Could not load courses. Error: ${e}`);
    clearUserCredentials();
  }
}

// Only relevant for web mode (not app mode)
async function loadAndDisplaySlots(sessionId, courseId) {
  const slots = await slotsWasm(sessionId, courseId);
  displaySlotsWeb(slots);
}
