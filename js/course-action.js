import {
  courses as coursesWasm,
  book_course as bookCourseSlotWasm,
  create_booking_request,
} from "../wasm-client/pkg/wasm_client.js";
import { fetchUserCredentials } from "./login.js";

const slotListEl = document.querySelector("#select-list");

slotListEl.addEventListener("click", bookCourseSlot);

export async function loadCourses(userCredentials) {
  if (userCredentials.mode === "app") {
    return coursesWasm(userCredentials["session"], userCredentials["user_id"]);
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
