<!--
SPDX-FileCopyrightText: 2025 Famedly GmbH (info@famedly.com)

SPDX-License-Identifier: Apache-2.0
-->

# Tracing Instrument Mock

[![rust workflow status][badge-rust-workflow-img]][badge-rust-workflow-url]
[![docker workflow status][badge-docker-workflow-img]][badge-docker-workflow-url]
[![docs main][badge-docs-main-img]][badge-docs-main-url]

[badge-rust-workflow-img]: https://github.com/famedly/rust-library-template/actions/workflows/rust.yml/badge.svg
[badge-rust-workflow-url]: https://github.com/famedly/rust-library-template/commits/main
[badge-docker-workflow-img]: https://github.com/famedly/rust-library-template/actions/workflows/docker.yml/badge.svg
[badge-docker-workflow-url]: https://github.com/famedly/rust-library-template/commits/main
[badge-docs-main-img]: https://img.shields.io/badge/docs-main-blue
[badge-docs-main-url]: https://famedly.github.io/rust-library-template/project_name/index.html

The `tracing::instrument` proc macro breaks the code coverage. There is an [open issue](https://github.com/tokio-rs/tracing/issues/2082) about it. While there's no solution for this issue, a workaround is to disable the instrument macro when tests with code coverage are run. his crate is just a mock of the `tracing::instrument` macro that does nothing and can replace the `tracing::instrument` macro when running with code coverage enabled.

You can select which version of the macro to use using the coverage configuration flag:

```rust
#[cfg(not(coverage))]
pub use tracing::instrument;
#[cfg(coverage)]
pub use tracing_instrument_mock::instrument;
```

Then, when instrumenting functions you can simply do

```rust
#[instrument]
fn my_function() {
  println!("Hello, world!");
}
```

## Lints

```sh
cargo clippy --workspace --all-targets
```

and this in your IDE:

```sh
cargo clippy --workspace --all-targets --message-format=json
```

## Pre-commit usage

1. If not installed, install with your package manager, or `pip install --user pre-commit`
2. Run `pre-commit autoupdate` to update the pre-commit config to use the newest template
3. Run `pre-commit install` to install the pre-commit hooks to your local environment
