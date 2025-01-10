# Argument Labels

Argument labels allow functions to specify what their arguments are for. These labels must be used when calling the function.

## Declaring Functions with Argument Labels

A function signature contains a list of parameters it takes. A parameter is declared like this: `name: Type`. A parameter is given a label by writing the label name before the declaration, like this: `label name: Type`. If the label and the name are the same, a shortcut of a underscore can be used for the label. A function with a mixture of all three styles would look like this:

```
func readAll(
  parentId: String,

  color c: Color,

  _ page: Int,
  _ pageSize: Int,
)
```

## Calling Functions with Argument Labels

There are two proposals for calling functions with argument labels. We are considering using swift-style colons, and python-style equals signs.

### Colon Style

Calling the `readAll` function with colon style would look like this:

```
readAll(id, color: .Red, page: 1, pageSize: 25)
```

When used as a markup style element, it would look like this:

```
Stack(
  gap: 4,
  alignment: .Center,
  justify: .Center
)
```

#### Advantages

- Easier to parse
- Less visual noise

#### Disadvantages

- Labels blend in more
- Looks worse for markup

### Equals Style

The two examples in equals style would look like this:

```
readAll(id, color = .Red, page = 1, pageSize = 25)
```

```
Stack(
  gap = 4,
  alignment = .Center,
  justify = .Center,
)
```

## Semantics of Argument Labels

Arguments must still be passed in the same order they are defined. If the wrong label is provided for an argument, or no label is provided when one is requested or vice versa, the compiler provides an error like this:

Error: expected argument label `label`, found `found label`.
Error: expected no argument label, found `found label`.
Error: expected argument label `label`, found none.

The compiler won't search the function for similar labels.
