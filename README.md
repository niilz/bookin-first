# Setup

## Lambda

Requires cargo lambda (docs to cargo lamda)[https://www.cargo-lambda.info/guide/getting-started.html]:

```
brew tap cargo-lambda/cargo-lambda
brew install cargo-lambda`
```

Create new function

`cargo lambda new <function-name>`

```
// on in WSL (docker image seems to need --http as option)
podman run --rm -v /mnt/c/Users/NHABERST/DEV/projects/booking-first:/booking-first cargo-lambda:latest sh -c "cd /booking-first && cargo lambda new courses-lambda --http"
```

Build function

`cargo lambda build --bin <function-dir> --release`

```
// on in WSL
podman run --rm -v /mnt/c/Users/NHABERST/DEV/projects/booking-first:/booking-first cargo-lambda:latest cargo lambda build --bin <function-dir> --release --manifest-path booking-first/Cargo.toml
```

Serve the function locally:

`cargo lambda watch --ignore-changes`

```
// on in WSL
podman run --rm -p 127.0.0.1:9000:9000 -v /mnt/c/Users/NHABERST/DEV/projects/booking-first:/booking-first cargo-lambda:latest cargo lambda watch --ignore-changes --manifest-path booking-first/Cargo.toml
```

Ignore changes might be neccessary, because some crates are just recompiling over and over again when function is invoked.
Dowside: requires rebuild

```
// Return LoginCreds (session-token and user-id)
curl -v -X POST \
  'http://127.0.0.1:9000/lambda-url/login-lambda/' \
  -H 'content-type: application/json' \
  -d '{ "user_name": "name", "password": "secret" }'
```

To deploy a lambda function to aws (enable function shows the URL):
`cargo lambda deploy --iam-role <arn-role-configured-in-aws-for-this> <lambda-function-package-name> --enable-function-url`
