# Control Flow

## If

Checks a condition, running one block of code if it is true and another if it is false.

### Syntax

```firefly
if condition {
    // run this code
}
else if other_condition {
    // run this code
}
else {
    // run this code
}
```

### Semantics

If the condition isn't a boolean, throw an error
After the if statement, an else statement can run a code block or another if statement
The first if statement which's condition is true runs
An if statement is a value with the type `()`


## While

## Return

Returns from the function early, possibly returning a value.

### Syntax

```firefly
return
return value
```

### Semantics

If the value returned is omitted, it will be assumed to be the unit tuple `()`.
The type of the value returned must match the return type of the function it is called in
Outside of a function, a return statement can't be used