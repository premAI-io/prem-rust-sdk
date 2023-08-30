# Prem Rust SDK

Rust SDK to Interact with Prem Ecosystem

## Generate Rust Types out of OpenAPI Spec

```bash
docker run --rm -v "${PWD}:/local" openapitools/openapi-generator-cli generate \
    -i /local/specs/chat.json \
    -g rust \
    -o /local/out/rust
```

## Local Installation

Put the package under your project folder in a directory named `prem` and add the following to `Cargo.toml` under `[dependencies]`:

```
prem = { path = "./prem-rust-sdk" }
```

## Documentation for API Endpoints

All URIs are relative to *http://localhost*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*DefaultApi* | [**chat_completions_v1_chat_completions_post**](docs/DefaultApi.md#chat_completions_v1_chat_completions_post) | **POST** /v1/chat/completions | Chat Completions
*DefaultApi* | [**health_v1_get**](docs/DefaultApi.md#health_v1_get) | **GET** /v1/ | Health


## Documentation For Models

 - [ChatCompletionInput](docs/ChatCompletionInput.md)
 - [ChatCompletionResponse](docs/ChatCompletionResponse.md)
 - [EmbeddingObject](docs/EmbeddingObject.md)
 - [EmbeddingUsage](docs/EmbeddingUsage.md)
 - [EmbeddingsInput](docs/EmbeddingsInput.md)
 - [EmbeddingsResponse](docs/EmbeddingsResponse.md)
 - [HealthResponse](docs/HealthResponse.md)
 - [HttpValidationError](docs/HttpValidationError.md)
 - [LocationInner](docs/LocationInner.md)
 - [Stop](docs/Stop.md)
 - [ValidationError](docs/ValidationError.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author

## Contributing

### Code Style and Linting

### Configure `pre-commit for automatic linting

Make sure you have `pre-commit` installed. You can check [here](https://pre-commit.com/#install) for installation instructions.

```bash
# install `pre-commit` hooks
pre-commit install
```

We use `clippy` to lint our code. To run the linter, use:

```bash
cargo clippy
```

We use `rustfmt` to format our code. To run the formatter, use:

```bash
cargo fmt
```

### Tests

Run the tests using:

```bash
cargo test
```