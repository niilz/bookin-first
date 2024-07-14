export function mapCourseSlots(courses, mode) {
  const courseSlotsMap = courses.reduce((map, c) => {
    const course = mode === "app" ? c.App.brief : c;
    // FIXME: If web mode ever works again, a web-course id can not be split
    const [courseId, slotId] = course.id.split(":");
    const courseKey = `${courseId}:${course.name}`;
    if (!map.has(courseKey)) {
      map.set(courseKey, []);
    }
    if (mode === "app") {
      const slot = mapSlot(course, slotId);
      map.get(courseKey).push(slot);
    }
    return map;
  }, new Map());
  return courseSlotsMap;
}

function mapSlot(course, slotId) {
  const { id, startDateTime, endDateTime, maxCapacity, totalBooked } = course;
  const [courseId] = id.split(":");
  const start = new Date(startDateTime);
  const end = new Date(endDateTime);

  const slot = {
    courseId,
    slotId,
    start,
    end,
    maxCapacity,
    totalBooked,
  };
  return slot;
}
