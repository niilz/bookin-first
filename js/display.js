export function displayCourses(courseSlots, courseListEl, mode) {
  console.log("Displaying courses");
  const courseListItems = courseSlots
    .entries()
    .map(([courseKey, slots], idx) => {
      const [id, name] = courseKey.split(":");
      if (mode === "web") {
        return `<li id="course-${idx}" class="course" data-course-id="${id}">${name}</li>`;
      } else if (mode === "app") {
        return `<li id="course-${idx}" class="course" data-course-id="${id}">
         <div>
            <p>${name}</p>
            ${slotsHtml(slots)}
         </div>
       </li>`;
      }
    });
  for (const course of courseListItems) {
    courseListEl.innerHTML += course;
  }
}

function slotsHtml(slots) {
  const html = slots
    .map(({ start, end }) => `<p>Start: ${start}, End: ${end}</p>`)
    .join("\n");
  return `<details>
        <summary>Slots</summary>
        ${html}
    </details>`;
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
