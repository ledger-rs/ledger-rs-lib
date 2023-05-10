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

# Documentation

- [Ledger for Developers](https://ledger-cli.org/doc/ledger3.html#Ledger-for-Developers)
- [Journal Format](https://ledger-cli.org/doc/ledger3.html#Journal-Format)
- Ledger source code [repo](https://github.com/ledger/ledger/)

## Formats

I will try to document the Ledger's Journal format in a [syntax diagram](http://www.plantuml.com/plantuml/duml/SoWkIImgIKtAI-FooYyjoalCKR1LY4XCBh7cSaZDIm590000).

![diagram](http://www.plantuml.com/plantuml/dsvg/LP1FIqCn3C3l-HIn7dJmla4PT0n4nCF4owwrZV9WRQ4l1MM-xsxIJdIN_CdlbnJg5lQLwIs_0yzy8doc-47rRi6dqchs9tmeDNa6-EstUDwNb2ZpEk7vw0El5a2peECZ-KXr-kySoi8oqAIcPZ5t8PIM5UoI5glkgFAtgtQ7whyqdtdchmikKMeHzkFQeSw1y1jVMx8mcYtU6sSc71Ss5eIDuTKKBezrP8tSCBSztlJep6P2faz6KHtbg5_r0m00)

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
