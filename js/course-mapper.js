export function mapCourseSlots(courses, mode) {
  const courseSlotsMap = courses.reduce((map, c) => {
    const course = mode === "app" ? c.App.brief : c;
    if (!map.has(course)) {
      map.set(course, []);
    }
    if (mode === "app") {
      const slot = mapSlot(course);
      map.get(course).push(slot);
    }
    return map;
  }, new Map());
  return courseSlotsMap;
}

function mapSlot(course) {
  const { startDateTime, endDateTime, maxCapacity, totalBooked } = course;
  const start = new Date(startDateTime);
  const end = new Date(endDateTime);

  const slot = {
    start,
    end,
    maxCapacity,
    totalBooked,
  };
  return slot;
}
