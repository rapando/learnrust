By default, rust runs all tests in parallel. We can change this by

```sh
cargo test -- --test-threads=1
```

This forces the tests to run on one thread, meaning one by one.

---

If you want to show the output of println! inside functions

```sh
cargo test -- --show-output
```

---

if you want to run a specific test,

```sh
cargo test name_of_test
```

If we set a partial name, it will run all matching names

---

If we use the `#[ignored]` flag in a test, its ignored unless its name is specified or
```sh
cargo test -- --ignored
```

---
if we only want to run the integration tests

```sh
cargo test --test integration_test
```
where `integration_test` is the name of the file in `tests/` directory 
