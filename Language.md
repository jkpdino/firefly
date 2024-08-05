# Version 0 Profile 1

- Code is organized into modules
- No generics
- All types are explicit
- All values are mutable
- Global variables
- Modules
    - Every file must have a module declaration
- Imports
    - An import takes all the symbols from one namespace and brings them to another
    - An import can also specify symbols or rename them with the as keyword
- Functions
    - No overloading
    - No parameter labels
    - Return types
- Structs
    - Have fields with visibilities
    - Have an initializer
    - Static methods
    - Instance methods
    - Static variables
- Control Flow
    - Loop/Repeat
    - If
    - While

# Version 0 Profile 2

- Structs
    - Initializers
- Functions
    - Parameter labels
    - Overloading
- Inference
    - Local bindings don't need an explicit type
- Operators

# Version 0 Profile 3

- Type aliases
- Protocols
- Generic types and functions

# By version 1

- New lang library
- Full type inference
- Enums
- Classes
- Pattern matching
- For and Match
- Closures


Example code

```firefly
// Version 0 Profile 1
module Fibonacci

func fibonacci(n: int) -> int {
    var i: int = 1;

    var n1: int = 1;
    var n2: int = 1;

    while lt_int(i, n) {
        var n3: int = add(n1, n2);

        n1 = n2;
        n2 = n3;

        i = add(i, 1);
    }

    return n2;
}
```

```firefly
// Version 0 Profile 2
module Fibonacci

func fibonacci(n: int) -> int {
    var i = 1;

    var n1 = 1;
    var n2 = 1;

    while i < n {
        var n3 = n1 + n2;

        n1 = n2;
        n2 = n3;

        i += 1;
    }

    return n2;
}
```