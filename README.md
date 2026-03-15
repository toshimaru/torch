[![.github/workflows/test.yml](https://github.com/toshimaru/torch/actions/workflows/test.yml/badge.svg)](https://github.com/toshimaru/torch/actions/workflows/test.yml)
[![Version](https://img.shields.io/crates/v/torch-cmd.svg)](https://crates.io/crates/torch-cmd)

# torch

`torch` = `mkdir` + `touch`.

`torch` command is `touch` a file with making directories.

## Motivation

When you want to create a file in a directory that does not exist, you need to create the directory first.

```console
$ mkdir -p path/to
$ touch path/to/file
```

I want to do this in one command.

It's a bit annoying to type `mkdir -p` and `touch` separately, so I created `torch` command.

## Install

### via Cargo

```console
$ cargo install torch-cmd
```

### via Homebrew

```console
$ brew install toshimaru/homebrew-torch/torch-cmd
```
