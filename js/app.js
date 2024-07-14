import init, { slots as slotsWasm } from "../wasm-client/pkg/wasm_client.js";
import { mapCourseSlots } from "./course-mapper.js";
import { displayCourses, displaySlotsWeb } from "./display.js";
import { loadCourses } from "./course-action.js";
import { fetchUserCredentials, login, loginFormEl } from "./login.js";

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

// Login-Inputs
const usernameInputEl = document.querySelector("#username-input");
const passwordInputEl = document.querySelector("#password-input");
// Displayed Data
const selectListEl = document.querySelector("#select-list");

async function tryLoadCourses() {
  const userCredentials = fetchUserCredentials();
  if (userCredentials) {
    return loadAndDisplayCourses(userCredentials, selectListEl);
  } else {
    throw Error("Please login befor loading courses");
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
const coursesButtonAppMode = document.querySelector("#courses-button-app-mode");
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
  console.log("Loading courses");
  let courseResult = await loadCourses(userCredentials).catch((e) => {
    console.log(`Could not load courses. Error: ${e}`);
    localStorage.clear();
    console.log("Cleared credentials");
  });
  const courseSlots = mapCourseSlots(courseResult, userCredentials.mode);
  displayCourses(courseSlots, selectListEl, userCredentials.mode);
  return courseResult;
}

// Only relevant for web mode (not app mode)
async function loadAndDisplaySlots(sessionId, courseId, selectListEl) {
  const slots = await slotsWasm(sessionId, courseId);
  displaySlotsWeb(slots, selectListEl);
}
