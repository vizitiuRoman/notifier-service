## Installation

- rustc [[installation]](https://www.rust-lang.org/en-US/install.html)
- cargo [[installation]](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- cargo-watch [[installation]](https://crates.io/crates/cargo-watch)

> Note: cargo is installed by default with a rustc installation

## Building from source

```bash
$ git clone https://github.com/vizitiuRoman/notifier-service.git
$ cd ./notifier-service/
$ cargo build --release
```

## Development
* Running local version in hot reload mode ``make watch``
* Linting project ``make lint``
* Unit tests ``make test``

## Docker
* Build docker image ``make docker-build``
* Build and push docker image ``make docker-publish``

### Branch naming

Branch names consist of a prefix, issue number, if any, and a short description of the issue in English (2-4 words).
The prefix is ​​the issue type (bug, feature), followed by a trailing slash `/`.

Examples:

    hotfix/ping-fix-response-code
    feature/ping-new-api-method
