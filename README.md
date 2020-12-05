# AdventOfCode
My personal repo for https://adventofcode.com/2020/

# Setup Rust
Install Rust on Linux / MacOS using Terminal

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Start a new shell window, run an update

```
rustup update
```

# Building, Testing, Running code
Git clone the repo

```
git clone https://github.com/ajit-kolathur/AdventOfCode.git;
cd AdventOfCode;
```

Build the project
```
cargo build
```

Testing

```
cargo test
```

Running a specific day

```
cargo run <space seperated dayN Ex: day1 day2 ...>
```

Running with info level logs

```
RUST_LOG=info cargo run day1
```

Running with debug level logs

```
RUST_LOG=debug cargo run day1
```