import {
  courses as coursesWasm,
  book_course as bookCourseSlotWasm,
  create_booking_request,
} from "../wasm-client/pkg/wasm_client.js";
import { fetchUserCredentials } from "./login.js";

const COURSES_KEY = "courses";
const ONE_DAY = 1000 * 60 * 60 * 24;

const slotListEl = document.querySelector("#select-list");

slotListEl.addEventListener("click", bookOrCancelCourseSlot);

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

export async function bookCourseSlot(event) {
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
    let { session, user_id: userId } = fetchUserCredentials();
    const booking = await bookCourseSlotWasm(bookingRequest, session, userId);
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
  console.log("Loading courses");
  return JSON.parse(coursesData);
}
