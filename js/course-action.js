import {
  courses as coursesWasm,
  book_course as bookCourseSlotWasm,
  create_booking_request,
} from "../wasm-client/pkg/wasm_client.js";
import { mapCourseSlots } from "./course-mapper.js";
import { fetchUserCredentials } from "./login.js";

const COURSES_KEY = "courses";
const ONE_DAY = 1000 * 60 * 60 * 24;

const slotListEl = document.querySelector("#select-list");

slotListEl.addEventListener("click", bookOrCancelCourseSlot);

let courseData;

export async function loadCourses(userCredentials, mode) {
  if (mode === "app") {
    const cachedCourses = getCourses();
    if (cachedCourses) {
      const { lastStored, courses } = cachedCourses;
      const sinceLastStored = Date.now() - lastStored;
      if (courses && sinceLastStored < ONE_DAY) {
        console.log("returning cached courses");
        const courseSlots = mapCourseSlots(courses, mode);
        return courseSlots;
      }
    }
    if (!userCredentials) {
      console.log("No cached courses requires Credentials");
      return;
    }
    console.log("No cached courses present: loading courses");
    const freshCourses = await fetchCourses(
      userCredentials["session"],
      userCredentials["user_id"]
    );
    if (freshCourses) {
      storeCourses(freshCourses);
    } else {
      console.warn("Did not receive any fresh courses to store");
      return;
    }
    const courseSlots = mapCourseSlots(freshCourses, mode);
    return courseSlots;
  } else if (mode === "web") {
    return fetchCourses(userCredentials.session, "");
  } else {
    throw Error(`Unsupported mode ${userCredentials.mode}`);
  }
}

export async function bookOrCancelCourseSlot(event) {
  const slot = event.target;
  const cssClasses = slot.classList;
  if (cssClasses.contains("slot")) {
    const slotData = getCourseData()[slot.id];
    const courseId = Number.parseInt(slot.dataset.courseId);
    const slotId = Number.parseInt(slot.dataset.slotId);
    const bookingRequest = create_booking_request(
      "42",
      slotId,
      courseId,
      "course-name-does-not-matter-in-app-mode"
    );
    const cancel = JSON.parse(slot.dataset.booked);
    let { session, user_id: userId } = fetchUserCredentials();
    const booking = await bookCourseSlotWasm(
      bookingRequest,
      session,
      userId,
      cancel
    );
    if (booking) {
      console.log({ slotData });
      // TODO: if success
      //  apply CSS and update SlotData (in memory)
      //  cssClasses.remove("booked");
      //  cssClasses.add("booked");
    }
    console.log({ booking });
  }
}

async function fetchCourses(session, userId) {
  try {
    const coursesResult = await coursesWasm(session, userId);
    const error = coursesResult.errorMessage;
    if (error) {
      console.warn(`Loading courses failed. Error: ${error}`);
      return;
    } else {
      return coursesResult;
    }
  } catch (e) {
    console.warn(`Loading courses failed. Error: ${e}`);
  }
}

export function setCourseData(newCourseData) {
  courseData = newCourseData;
}

export function getCourseData() {
  return courseData;
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
clearCoursesButton.addEventListener("click", () => {
  window.localStorage.removeItem(COURSES_KEY);
  console.log("Cleared course cache");
});
