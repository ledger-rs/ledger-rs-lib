# ledger-rs-lib
Ledger-cli functionality implemented in Rust

A brand new attempt at Ledger, starting from blank slate.

[Early Work In Progress!]

![](https://img.shields.io/crates/v/ledger-rs-lib?style=plastic)

# Introduction

This library is aiming to implement the plain-text accounting principles, as demonstrated by Ledger-cli, Hledger, Beancount, etc. The base for development is [Ledger-cli](https://github.com/ledger/ledger/), from which the underlying model and concepts were applied.

Part of the [Rusty Ledger](https://github.com/ledger-rs/) project.

# Current State

While still early work-in-progress, the basic functionality is there. Simple Ledger journal files with transactions are being parsed. The basic functionality allows retrieving the parsed entities (Transactions, Posts, Accounts, Cost, Amounts), which allow the very basic reports to be provided.

The functionality will now be expanded. The direction will be tracked through work items (issues) in the repository.

Any kind of contribution is wellcome, from using the library and providing feedback, to participating in discussions and submitting pull-requests with implementations and improvements.

# Background

After a few attempts at rewriting (pieces of) Ledger, the following conclusions seemed to crystalize:

1. One package
While trying to create just a parser, it became clear that there is no need to separate the parser from the rest of the application (model, reports). They can coexist in the same crate. The parser can still be used independently of other functionality.
The model and the reports are easier to include in the same crate from the beginning. These can be separated if ever needed.

1. Clean Rust
Trying to convert the C++ structure into Rust just doesn't make much sense. The pointer arithmetic in the original Ledger seems next to impossible to create and maintain in Rust. The references and lifetimes make development a nightmare in Rust. A clean start, applying idiomatic Rust concepts should apply.

1. Start minimal
Ledger really contains a lot of features. Start from a minimal working version and expand from there.

1. Define clear goals
Trying to rewrite the whole application seems a neverending task. Rather, define clear and small, attainable goals and implement them.

# Goals

The goals beyond the initial requirements, which served as a proof-of-concept, will be tracked as issues in the source repository.

## Initial functional requirements

The immediate goals are:

- [x] Parse a minimal working transaction sample
- [x] Create a minimal working reports:
  - [x] Accounts
  - [x] Balance
- [ ] Compile a working WASM version
  - [ ] that interacts with JavaScript
  - [x] that works in console

These should provide initial insights into Ledger's inner workings and concepts.

## Non-Functional Requirements

- Fast execution
- Test coverage

## Experimental Goals

- Permanent storage (sqlite?) as a base for the reporting layer

# WASM/WASI

## WASI

To compile to Wasm for execution in WASI, run
```
cargo build --target wasm32-wasi
```
then go to the `target/wasm32-wasi/debug` folder and run
```
wasmer run --dir tests target/wasm32-wasi/debug/ledger-rs-lib.wasm -- -f tests/minimal.ledger
```

This will run the CLI's main() method. With WASI, the filesystem access permission has to be given explicitly. This is done with `--dir` argument.
Note that Wasmer is using `--` to separate application switches like `-f`.

You need to have the prerequisites install - the compilation target (wasm32-wasi) and a Wasm runtime (i.e. wasmer).

## WASM 

The library can be compiled into WASM for use in web apps.

```shell
cargo install wasm-pack

wasm-pack build --target web
```

### Demo

The folder `wwwroot` contains the code that uses the wasm.
Serve with a web server. I.e. using Deno's file server:
```
deno install --allow-net --allow-read https://deno.land/std/http/file_server.ts

file_server wwwroot
```
Add the deno plugin location to path.


# Documentation

- [Ledger for Developers](https://ledger-cli.org/doc/ledger3.html#Ledger-for-Developers)
- [Journal Format](https://ledger-cli.org/doc/ledger3.html#Journal-Format)
- Ledger source code [repo](https://github.com/ledger/ledger/)

## Journal Format

I will try to document the Ledger's Journal format in a [syntax diagram](http://www.plantuml.com/plantuml/duml/LL9HhjCm4FptAHRpGLAv5o2gbAWLGeYF88fKgOgGIKnnIMpaR52hqBjm6Ux5RkAsBp_jxkpCsBDEtgCEQBwvxm8jjWO-ckPamfiUFlWXEDt2EnywZUA7qOq9KBJ6mR-_jZthdogIrw67Ny6VJOr2t6KR6BU-wup3cuBne6kyPK8aA-0ITZOGs_usi4e58yG_l9-EK2-5fU-H0FvZUQGGUQVHA3WMm-KhbnNLSYNX3yXNafj49bB1rZV4agbC6IlrrKpCw5zbWet9hQXhFpXamuwBYYldF6gqtaqI8Ywbd8Nbrfqun6onC1iJ-LQgUvzIWAABd4-3TcZn6XrzGpLvFiya3cKOILwQyCLPB8EjESjDffIIHZpR4xjzJ6WqHpzA5HSaAy8oiPrZJanIVnwwJCpDXgnoeiytIpD1inbSe2BcdaRPjEVNSTlycyjK0PeBGjmBUoyVUOBMJsW3idnSyxYt7R_CSosFhP1XRbp3N-X_). This is the Extended Backus–Naur Form (EBNF) diagram source.

![diagram](http://www.plantuml.com/plantuml/dsvg/LL9HhjCm4FptAHRpGLAv5o2gbAWLGeYF88fKgOgGIKnnIMpaR52hqBjm6Ux5RkAsBp_jxkpCsBDEtgCEQBwvxm8jjWO-ckPamfiUFlWXEDt2EnywZUA7qOq9KBJ6mR-_jZthdogIrw67Ny6VJOr2t6KR6BU-wup3cuBne6kyPK8aA-0ITZOGs_usi4e58yG_l9-EK2-5fU-H0FvZUQGGUQVHA3WMm-KhbnNLSYNX3yXNafj49bB1rZV4agbC6IlrrKpCw5zbWet9hQXhFpXamuwBYYldF6gqtaqI8Ywbd8Nbrfqun6onC1iJ-LQgUvzIWAABd4-3TcZn6XrzGpLvFiya3cKOILwQyCLPB8EjESjDffIIHZpR4xjzJ6WqHpzA5HSaAy8oiPrZJanIVnwwJCpDXgnoeiytIpD1inbSe2BcdaRPjEVNSTlycyjK0PeBGjmBUoyVUOBMJsW3idnSyxYt7R_CSosFhP1XRbp3N-X_)

The original specs from Ledger's documentation:

Transaction header
```
DATE[=EDATE] [*|!] [(CODE)] DESC
```

Posting
```
  ACCOUNT  AMOUNT  [; NOTE]
```

Price
```
P DATE SYMBOL PRICE
```

## Lots

The price of a commodity is stored in commodity annotations (`amount.h`).

`annotate_commodity(amount_t price, [datetime_t date, string tag])`
