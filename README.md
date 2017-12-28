# xb_automata

This is an example of a Universal Turing Machine called the __XB__ machine, inspired by Gary Bernhardt's screencasts

## Architecture

The architecture for this machine is relatively simple.

Our universal machine is comprised of two cells:
     
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

## Installation

This machine / architecture / language / whatever is built as a Rust library. Not really something to be implemented for pragmatism,but rather for learning and understanding theoretical computer science.

To see how it works, run the only available test:

    cargo test -- --nocapture

## License

[MIT License](https://codemuch.tech/license.txt)