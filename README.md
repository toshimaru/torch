[![Test](https://github.com/toshimaru/torch/actions/workflows/test.yml/badge.svg)](https://github.com/toshimaru/torch/actions/workflows/test.yml)
[![Version](https://img.shields.io/crates/v/torch-cmd.svg)](https://crates.io/crates/torch-cmd)

# torch

`torch` is a small CLI that combines `mkdir -p` and `touch`.

It creates parent directories when needed, then creates the file.

## Why

Creating a file in a nested path usually takes two commands:

```console
$ mkdir -p path/to
$ touch path/to/file.txt
```

`torch` reduces that to one:

```console
$ torch path/to/file.txt
```

## What it does

Given one or more paths, `torch`:

- creates missing parent directories
- creates the target file if it does not exist
- creates the path itself as a directory when it ends with a `/`
- updates the access and modification times if it already exists
- exits with a non-zero status if any path fails

## Usage

```console
$ torch <PATH>...
```

Examples:

```console
$ torch notes/today.md
$ torch app/models/user.rb app/controllers/users_controller.rb
$ torch tmp/output.log
```

A path ending with `/` is created as a directory instead of a file:

```console
$ torch app/services/
```

is roughly equivalent to:

```console
$ mkdir -p app/services
```

This command:

```console
$ torch docs/guides/getting-started.md
```

is roughly equivalent to:

```console
$ mkdir -p docs/guides
$ touch docs/guides/getting-started.md
```

To see the built-in CLI help:

```console
$ torch --help
```

## Install

### Cargo

```console
$ cargo install torch-cmd
```

### Homebrew

```console
$ brew install toshimaru/homebrew-torch/torch-cmd
```

## Development

Run the test suite:

```console
$ cargo test
```

Run the CLI locally:

```console
$ cargo run -- path/to/file.txt
```

## License

MIT
