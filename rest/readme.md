## A Simple REST API with RUST

Compile and run

```sh
cargo build --release
./target/release/rest
```


## To run load tests

1. Install artillery

```sh
sudo npm install -g artillery@latest
```

2. Run the tests

```sh
source .env
artillery run api-load-test.yml --record --key ${ARTILLERY_API_KEY}
```

3. Generate HTML report

```sh
artillery report --output report.html --input report.json
```
