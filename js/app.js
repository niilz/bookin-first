import init, { login, courses, slots } from "../wasm-client/pkg/wasm_client.js";

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
const courseList = document.querySelector("#course-list");

courseList.addEventListener("click", (e) => {
  const course = e.target;
  if (course.classList.contains("course")) {
    // TODO: map course to data or backing array
    console.log(`Clicked cours: ${course}`);
    console.log("TODO: fetch slots");
  }
});

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
  const courseResult = await courses(session);
  displayCourses(courseResult);

  console.log({ courses });
});

function displayCourses(courses) {
  const courseListItems = courses.map((course) => {
    let { id, title } = course;
    return `<li id="course-${id}" class="course">${title}</li>`;
  });
  for (const course of courseListItems) {
    courseList.innerHTML += course;
  }
}
