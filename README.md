# xb

xb is a simple Universal Turing Machine with an even simpler instruction set.

spec
====

The architecture for this machine is relatively simple.

Our universal machine is comprised of a "tape" with two cells:
     
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

Some other resources:

* What is a UTM - https://en.wikipedia.org/wiki/Universal_Turing_machine
* The inspiration - https://www.destroyallsoftware.com/screencasts/catalog/power-of-turing-machines

install
=======

This project is distributed as a Rust library.

To see how it works, run the only available test:

    cargo test

license
=======

[mit](https://codemuch.tech/license.txt)
