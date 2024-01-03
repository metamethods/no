# unix-no

[![Repository Stars](https://img.shields.io/github/stars/metamethods/no)](https://github.com/metamethods/no)
[![Repository Forks](https://img.shields.io/github/forks/metamethods/no)](https://github.com/metamethods/no/forks)
[![Crates.io Downloads](https://img.shields.io/crates/d/unix-no)](https://crates.io/crates/no)
[![Crates.io Version](https://img.shields.io/crates/v/unix-no)](https://crates.io/crates/no)
[![License](https://img.shields.io/github/license/metamethods/no)](https://github.com/metamethods/no/blob/master/LICENSE)

A rust implementation of the unix `no` command which is the counterpart command of the `yes` command, and a bit of a better version of this [one](https://crates.io/crates/no)

## Description
Repeatedly output a line with all specified STRING(s), or 'n'.

## Usage
```bash
no [STRINGS]
```

## Building
### Prerequisites
* [Rustup](https://rustup.rs/)

### Instructions
1. Clone this repository via `git clone https://github.com/metamethods/no`
2. Build with `cargo` using `cargo build --release`
3. The binary will be located at `target/release/unix-no` (or `target/release/unix-no.exe` if you're using windows)

## TODO
- [ ] Figure out how to create a man page for this