# ledger-rs-prototype
Brand new attempt at Ledger from blank slate

Work In Progress

# Background

After a few attempts at rewriting (pieces of) Ledger, some conclusions were made:

1. Whole package.
While trying to create a separate parser, it became clear that there is no need to separate the parser from the rest of the application. The parser will be available for use independently, however.
The model and the reports are easier to include in the same crate from the beginning.

2. Clean Rust.
Trying to convert the C++ structure into Rust just doesn't make any sense. The pointer arithmetic in the original Ledger seems next to impossible to create and maintain in Rust. A clean start, using Rust concepts should apply.

3. Start minimal.
Ledger really contains a lot of features. Start from a minimal working version and expand from there.

4. Define clear goals.
Trying to rewrite the whole application seems a neverending task. Rather, define clear and small, attainable goals.

# Goals

The immediate goals are:

- [ ] Parse a minimal working transaction sample
- [ ] Create a minimal working Balance report
- [ ] Compile a working WASM version that interacts with JavaScript

These should provide insights into Ledger's inner workings and concepts.
