# Control Flow

## If

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