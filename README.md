#  Guess Number In Rust
A small command-line number guessing game written in Rust.

You pick the range, the program picks a secret number inside it, and you try to find it. After each guess you're told whether the secret number is bigger or smaller, and at the end you see how many tries it took.

## Features
`Custom range`: you choose the lowest and highest possible number.

`Input validation`: non-numeric input and out-of-range guesses are rejected with a helpful message and don't count as a try.

`Hints`: after each guess (bigger / smaller).

`Try counter`: show when you win.

## Requirements
- [Rust](https://rust-lang.org/tools/install/) (stable toolchain, installed with `rustup`)


Check that it's installed:
```
cargo --version
```

## Getting Started
Clone this repository and run the game:
```
git clone https://github.com/NamelessProj/Guess-Number-Rust.git
cd Guess-Number-Rust
cargo run
```

To build an optimized binary instead:
```
cargo build --release
```

The executable is created in `target/release`.

## How To Play
```
From:
1

To:
10

Guess the number between 1 and 10

Type your guess:
5
Sorry, but you're looking for a bigger number than: 5

Type your guess:
8
Great job!
You found the number. It was: 8
And it only took you 2 tries
```

## Dependencies
- [`rand`](https://crates.io/crates/rand): random number generation

## Project Structure
```
.
├── Cargo.toml
└── src
    └── main.rs
```

## Contributing
This is a learning project, but suggestions and PRs are welcome. Feel free to open an issue.

## License
This project is licensed under the [MIT License](LICENSE).