# ocaml-btreemap

An [ocaml-rs](https://github.com/zshipko/ocaml-rs) wrapper around the Rust [`BTreeMap`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html) type for use in OCaml

## Building

    make

to run the tests:

    make test

or to build manually:

    cargo build
    jbuilder build
    jbuilder runtest

## Installation

    opam pin add btreemap .


## API

See `src/btreemap.mli`

