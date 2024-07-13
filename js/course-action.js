import { courses as coursesWasm } from "../wasm-client/pkg/wasm_client.js";

const slotListEl = document.querySelector("#select-list");

slotListEl.addEventListener("click", bookCourseSlot);

export async function loadCourses(userCredentials) {
  if (userCredentials.mode === "app") {
    return coursesWasm(userCredentials["session"], userCredentials["user_id"]);
  } else if (userCredentials.mode === "web") {
    return coursesWasm(userCredentials.session, "");
  } else {
    throw Exception(`Unsupported mode ${userCredentials.mode}`);
  }
}

export function bookCourseSlot(event) {
  const element = event.target;
  const classes = element.classList;
  if (classes.contains("slot")) {
    const slotId = element.dataset.slotId;
  }
}
