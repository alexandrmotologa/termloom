# Contributing to TermLoom

Thank you for your interest in contributing to TermLoom.

## Development Setup

### Prerequisites

* Rust 1.75 or newer (`cargo`, `rustc`)
* Git
* On Linux: development packages for `libpty` or POSIX headers if compiling natively

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test --all
```

### Checking Formatting and Lints

```bash
cargo fmt --check
cargo clippy -- -D warnings
```

## Submitting Pull Requests

1. Fork the repository on GitHub.
2. Create a feature branch: `git checkout -b feature/my-new-feature`.
3. Ensure all tests pass and documentation is updated.
4. Commit your changes with clear, descriptive commit messages following standard conventions (`feat:`, `fix:`, `docs:`, `perf:`).
5. Push your branch and open a Pull Request.

## Code Standards

* Write clear, readable Rust code adhering to standard formatting (`rustfmt`).
* Maintain clean cross-platform compatibility across Windows, macOS, and Linux.
* Avoid adding large third-party runtime dependencies without discussion.
