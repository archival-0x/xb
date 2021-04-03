# xb

xb is a simple Turing Machine with an even simpler instruction set.

## Introduction

This is a proof-of-concept of an abstraction implementation for a Turing Machine.

Our machine is comprised of a "tape" with two cells:

    | B | B |

The purpose of this machine is to make it such that it is capable of changing the first cell to an X, and then reverting back to a B.

    Pseudocode Instruction Set:
        R       = move cell to the right
        L       = move cell to the left
        ->      = alter content of cell
        s1..s4  = state symbols

    Our example program:
        B, s1 -> X, R, s2
        B, s2 -> B, L, s3
        X, s3 -> B, R, s4
        B, s4 -> B, L, s1

## Build

This project is distributed as a Rust library.

To see how it works, run the only available test:

```
$ cargo test
```
