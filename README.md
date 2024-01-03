# no
Output a string repeatedly until killed.

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