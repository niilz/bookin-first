// Displayed Data
const selectListEl = document.querySelector("#select-list");

export function displayCourses(courseSlots, mode) {
  console.log("Displaying courses");
  selectListEl.innerHTML = "";
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
            ${slotsHtml(name, slots)}
         </div>
       </li>`;
      }
    });
  for (const course of courseListItems) {
    selectListEl.innerHTML += course;
  }
}

// APP-Mode
function slotsHtml(courseName, slots) {
  const html = slots
    .map(
      (
        { courseId, slotId, start, end, maxCapacity, totalBooked, booked },
        idx
      ) =>
        `<p id="slot-${idx}"
            class="slot ${booked ? "booked" : ""}"
            data-course-id="${courseId}:${courseName}"
            data-slot-id="${slotId}">
                Start: ${start}
                <br>
                End: ${end}
                <br>
                capacity: ${maxCapacity}
                <br>
                total booked: ${totalBooked}
      </p>`
    )
    .join("\n");
  return `<details>
        <summary>Slots</summary>
        ${html}
    </details>`;
}

// WEB-Mode
export function displaySlotsWeb(slots) {
  selectListEl.innerHTML = "";
  const slotListItems = slots.map((slot, idx) => {
    let { id, start_date_time, booked_participants, max_participants } = slot;
    return `<li id="slot-${idx}" class="slot" data-slot-id="${id}">start: ${start_date_time}, participants: ${booked_participants}, max: ${max_participants}</li>`;
  });
  for (const slot of slotListItems) {
    selectListEl.innerHTML += slot;
  }
}
