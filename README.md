# Setup

## Lambda

Requires cargo lambda (docs to cargo lamda)[https://www.cargo-lambda.info/guide/getting-started.html]:

```
brew tap cargo-lambda/cargo-lambda
brew install cargo-lambda`
```

Build function

`cargo lambda build --bin <function-dir> --release`

Serve the function locally:

`cargo lambda watch --ignore-changes`

Ignore changes might be neccessary, because some crates are just recompiling over and over again when function is invoked.
Dowside: requires rebuild
