# TODO

## Required

- Report actual errors on wasm, lambda and rust side
  - Handle errors e.g. when Slot can not be read (is it a parsing problem?) or when Course is not bookable
  - handle error when booking/canceling fails
- Make lambda functions hosts configurable
- Make gym-location (returned in login response) variable in fetch courses
- add some better design
- Ensure that course loading still works with Web mode (serialization might fail in fitness_service#fetch_courses)

## Super Cool

- Add appointment to calendar

## Nice to have

- port more js parts to rust
- implement unsupervised run (for course and slot)
