export function displayCourses(courses, courseListEl, mode) {
  console.log("Displaying courses");
  const courseListItems = courses.map((course, idx) => {
    if (mode === "web") {
      let { id, title } = course;
      return `<li id="course-${idx}" class="course" data-course-id="${id}">${title}</li>`;
    } else if (mode === "app") {
      let { id, name } = course.App.brief;
      return `<li id="course-${idx}" class="course" data-course-id="${id}">${name}</li>`;
    }
  });
  for (const course of courseListItems) {
    courseListEl.innerHTML += course;
  }
}

export function displaySlots(slots, slotListEl) {
  slotListEl.innerHTML = "";
  const slotListItems = slots.map((slot, idx) => {
    let { id, start_date_time, booked_participants, max_participants } = slot;
    return `<li id="slot-${idx}" class="slot" data-slot-id="${id}">start: ${start_date_time}, participants: ${booked_participants}, max: ${max_participants}</li>`;
  });
  for (const slot of slotListItems) {
    slotListEl.innerHTML += slot;
  }
}
