# TODO

## Required

- Handle different course-layouts in app and web mode (in JS)
- Ensure that course loading still works with Web mode (serialization might fail in fitness_service#fetch_courses)
- Show slots from app-mode resonse
- implement book course with app-mode
- Make gym-location (returned in login response) variable in fetch courses
- figure out how to handle store local-storage credentials
- Why is slots not working? (can you inspect phone with wire shark?)
- fetch slots via JS
- book slot via JS
- Make lambda functions hosts configurable
- deploy functions to aws
- Handle errors e.g. when Slot can not be read (is it a parsing problem?) or when Course is not bookable

## Super Cool

- Add appointment to calendar

## Nice to have

- Remove trailing String parts on PHPSESS-Cookie (not sure if that is still the case)
- Store token and session (cache)
- implement delete
- implement unsupervised run (for course and slot)
