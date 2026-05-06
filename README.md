# Learning Rust

A personal learning project following [The Rust Programming Language](https://doc.rust-lang.org/stable/book/) (a.k.a. "The Book").

## Projects

| Directory | Book Chapter | Topic |
|---|---|---|
| `hello_cargo/` | Ch. 1 — Getting Started | Hello world with Cargo; basic I/O (early guessing game sketch) |
| `src/` (root) | Ch. 2 — Programming a Guessing Game | Complete guessing game with random numbers, loops, and match |
| `variables/` | Ch. 3 — Common Programming Concepts | Variables, mutability, and shadowing |

## Running a Project

Make sure you have [Rust installed](https://www.rust-lang.org/tools/install), then use Cargo:

```bash
# Run the guessing game (root project)
cargo run

# Run a sub-project
cargo run --manifest-path hello_cargo/Cargo.toml
cargo run --manifest-path variables/Cargo.toml
```

## Progress

- [x] Ch. 1 — Getting Started
- [x] Ch. 2 — Programming a Guessing Game
- [x] Ch. 3 — Common Programming Concepts (Variables & Shadowing)
- [ ] Ch. 4 — Understanding Ownership
- [ ] Ch. 5 — Using Structs
- [ ] Ch. 6 — Enums and Pattern Matching
- [ ] ...

## Reference

- Book: https://doc.rust-lang.org/stable/book/
- Standard library docs: https://doc.rust-lang.org/std/
- Cargo docs: https://doc.rust-lang.org/cargo/
