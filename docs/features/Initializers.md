# Initializers

## Description

Initializers are special methods used to create and set up new instances of a struct. They ensure that all fields of a struct are properly initialized before the instance can be used.

## Syntax

````firefly
visibility init(parameters) {
  // body
}

Initializers can use argument labels like regular functions:

```firefly
public func init(name: string, age: int) {
  self.name = name;
  self.age = age;
}
````

## Semantics

- Initializers are declared with the `init` keyword inside a struct.
- Initializers can have visibility modifiers (public, internal, fileprivate, private), defaulting to internal when not specified.
- Initializers always return an instance of their struct (implicitly, no return statement needed).
- All fields of the struct must be set within the initializer before completion.
- Fields cannot be accessed or used before they are initialized.
- An initializer is called using the struct name followed by parentheses and arguments:
- Argument labels must be used when calling an initializer, following the same rules as regular functions.

## Examples

### Defining an initializer

```firefly
public struct Person {
    var name: String
    var age: Int

    // Basic initializer
    public init(name: String, age: Int) {
        self.name = name
        self.age = age
    }

    // Initializer with default values
    init(name: String) {
        self.name = name
        self.age = 0
    }
}
```

### Calling an initializer

```firefly
let person1 = Person(name: "Alice", age: 30)
let person2 = Person(name: "Bob")
```

## Error handling

- Compiler will generate an error if any field is not initialized in an initializer
- Attempting to use a field before it's initialized will result in a compile-time error
