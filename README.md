# Rustry (pls help I'm bad at naming)

<insert_name> is a test framework for smart contract for writing tests in Rust.

## Why ?

- Benefit from Rust language features. 
    - Native multithreading for heavy scenarios from which the execution is independent from each iteration
    - Out-of-the-box fuzzing
- Not writing tests in a language that compiles to EVM, which slows things down and add constraints
- Less boilerplate. Macros can do a lot of things !

## Design choices

A workspace, because of Rust limitations about exporting modules from a proc-macro crate.

I'm also trying to limit as much as possible compilation times. Nobody wants to have to wait ages to run a test again. 
Lightweight versions of packages are then rewritten to remove the bloat.

Don't close the user in tool-specific features that they would have to learn again, but use widely adopted stuffs.

## How to use it ?

See `examples/`

To run the examples, launch

```sh
cargo test --workspace --examples
# or for a single example
cargo test --workspace --example counter
```
