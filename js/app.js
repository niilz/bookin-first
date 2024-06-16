import init, {
  login as loginWasm,
  courses as coursesWasm,
  slots as slotsWasm,
} from "../wasm-client/pkg/wasm_client.js";

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
          loginForm.classList.add("hidden");
        }
      })
      .catch((e) => {
        console.error(`unexpected error during initial course loading: ${e}`);
        // show login form if courses could not be loaded
        loginForm.classList.remove("hidden");
      });
  })
  .catch((e) => console.error(`init failed ${e}`));

let userCredentials;

// Login-Inputs
const loginForm = document.querySelector("#login-form");
const usernameInput = document.querySelector("#username-input");
const passwordInput = document.querySelector("#password-input");
// Displayed Data
const courseList = document.querySelector("#course-list");

async function tryLoadCourses() {
  const userCredentialsString = localStorage.getItem(USER_CREDENTIALS);
  if (userCredentialsString) {
    userCredentials = JSON.parse(userCredentialsString);
    console.log({ userCredentials });
    return await loadAndDisplayCourses(userCredentials.session);
  } else if (loginForm.classList.contains("hidden")) {
    // Show login-form if credentials are not present
    loginForm.classList.remove("hidden");
  }
}

courseList.addEventListener("click", (e) => {
  const course = e.target;
  if (course.classList.contains("course")) {
    // TODO: map course to data or backing array
    console.log(`Clicked cours: ${course}`);
    console.log("TODO: fetch slots");
  }
});

const coursesButton = document.querySelector("#courses-button");
coursesButton.addEventListener("click", async (e) => {
  e.preventDefault();
  if (!userCredentials) {
    const username = usernameInput.value;
    const password = passwordInput.value;
    login(username, password);
  }
  const { session } = userCredentials;
  loadAndDisplayCourses(session);
});

async function login(username, password) {
  userCredentials = await loginWasm(username, password);
  localStorage.setItem(USER_CREDENTIALS, JSON.stringify(userCredentials));

  console.log({ userCredentials });
}

async function loadAndDisplayCourses(sessionId) {
  const courseResult = await coursesWasm(sessionId);
  displayCourses(courseResult);
  return courseResult;
}

function displayCourses(courses) {
  const courseListItems = courses.map((course, idx) => {
    let { id, title } = course;
    return `<li id="course-${idx}" class="course" data-course-id="${id}">${title}</li>`;
  });
  for (const course of courseListItems) {
    courseList.innerHTML += course;
  }
}
