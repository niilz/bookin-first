import {
  courses as coursesWasm,
  book_course as bookCourseSlotWasm,
  create_booking_request,
} from "../wasm-client/pkg/wasm_client.js";
import { fetchUserCredentials } from "./login.js";

const COURSES_KEY = "courses";
const ONE_DAY = 1000 * 60 * 60 * 24;

const slotListEl = document.querySelector("#select-list");

let courseData;

slotListEl.addEventListener("click", bookOrCancelCourseSlot);

//export async

export async function loadCourses(userCredentials) {
  if (userCredentials.mode === "app") {
    const cachedCourses = getCourses();
    if (cachedCourses) {
      const { lastStored, courses } = cachedCourses;
      const sinceLastStored = Date.now() - lastStored;
      if (courses && sinceLastStored < ONE_DAY) {
        console.log("returning cached courses");
        return courses;
      }
    }
    console.log("No cached courses present: loading courses");
    const freshCourses = await coursesWasm(
      userCredentials["session"],
      userCredentials["user_id"]
    );
    storeCourses(freshCourses);
    return freshCourses;
  } else if (userCredentials.mode === "web") {
    return coursesWasm(userCredentials.session, "");
  } else {
    throw Error(`Unsupported mode ${userCredentials.mode}`);
  }
}

export async function bookOrCancelCourseSlot(event) {
  const slot = event.target;
  const cssClasses = slot.classList;
  if (cssClasses.contains("slot")) {
    const slotId = Number.parseInt(slot.dataset.slotId);
    const courseId = Number.parseInt(slot.dataset.courseId);
    const bookingRequest = create_booking_request(
      "42",
      slotId,
      courseId,
      "course-name-does-not-matter-in-app-mode"
    );
    const cancel = JSON.parse(slot.dataset.booked);
    if (cancel) {
      cssClasses.remove("booked");
      slot.dataset.booked = false;
    } else {
      slot.dataset.booked = true;
      cssClasses.add("booked");
    }
    let { session, user_id: userId } = fetchUserCredentials();
    const booking = await bookCourseSlotWasm(
      bookingRequest,
      session,
      userId,
      cancel
    );
    console.log({ booking });
  }
}

function storeCourses(courses) {
  const lastStored = Date.now();
  console.log("Storing courses");
  const coursesData = JSON.stringify({ lastStored, courses });
  window.localStorage.setItem(COURSES_KEY, coursesData);
}

function getCourses() {
  const coursesData = window.localStorage.getItem(COURSES_KEY);
  console.log("Retrieving courses from local storage");
  return JSON.parse(coursesData);
}

const clearCoursesButton = document.querySelector("#clear-courses-button");
clearCoursesButton.addEventListener("click", () =>
  window.localStorage.removeItem(COURSES_KEY)
);
