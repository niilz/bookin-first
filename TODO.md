# TODO

## Required

- implement delete (at least for app-mode almost like book, just instead addExerciser use removeExerciser)
- figure out how to "better" handle store local-storage credentials (like clearing)
- add some better design
- Ensure that course loading still works with Web mode (serialization might fail in fitness_service#fetch_courses)
- Make gym-location (returned in login response) variable in fetch courses
- Make lambda functions hosts configurable
- deploy functions to aws
- Handle errors e.g. when Slot can not be read (is it a parsing problem?) or when Course is not bookable

## Super Cool

- Add appointment to calendar

## Nice to have

- implement unsupervised run (for course and slot)
