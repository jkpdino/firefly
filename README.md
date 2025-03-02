# Firefly

Firefly is a very basic programming language and compiler. Firefly supports modern syntax and basic procedural programming features. Over time, firefly will incorporate features of functional and object oriented programming languages and become much more usable.

## Quick Peek

In its current state, firefly can be used for basic mathematical functions.

```firefly
module Fibonacci

func fibonacci(n: int) -> int {
    var i = 1;

    var n1 = 1;
    var n2 = 1;

    while i < n {
        var n3 = n1 + n2;

        n1 = n2;
        n2 = n3;

        i = i + 1;
    }

    return n2;
}
```

## Running

To run firefly, use the CLI with the list of files to compile as arguments. Your program will run in an interpreter. The `--print-hir` flag dumps the hir tree to the console, and `--print-mir` dumps mir.
