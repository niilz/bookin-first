# TODO

## Required

- finish wasm-wiring (like cli) to call lambda-functions with fetch
  - have fetch-wrapper (and tests)
- implement HTML/JS UI
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
