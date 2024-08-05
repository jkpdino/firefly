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

Runs a block of code while a condition is true. If the condition is initially false, never runs the code.

### Syntax

```firefly
while condition {

}

label: while condition {

}
```

### Semantics

- Throws an error if condition is not a boolean
- Runs the code while the condition is true
- Evaluates the condition on each run-through
- Able to be exited early with the break keyword
- Using a label allows a specific loop to be exited
- Labels are available within their block
- Labels shadow other, outside labels with the same name

## Break

A break statement exits from a loop. It can specify which loop to exit, or else it will exit the innermost loop.

### Syntax

```firefly
break
break label
```

### Semantics

- If it isn't in a loop, an error is thrown
- If the label doesn't exist, an error is thrown
- A break value has the never type

## Break

A continue statement moves to the next iteration of a loop. It can specify which loop to continue, or else it will continue the innermost loop.

### Syntax

```firefly
continue
continue label
```

### Semantics

- If it isn't in a loop, an error is thrown
- If the label doesn't exist, an error is thrown
- A continue value has the never type

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