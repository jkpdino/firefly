# Structs

## Description

A struct provides a new datatype bundling a collection of other datatypes. A struct can also have associated methods, static functions, and static variables.

## Syntax

```firefly
visibility Foo {
    items
}
```

```
visibility var foo: int
```

```
parent.field_name
```

```
Foo("hello, world")
```

```
parent.fooBar("Hello, World")
```

## Semantics

A struct is made up of a collection of fields. These fields are declared using a variable declaration, which will throw an error if it is given a default value. Fields can be accessed using member syntax, with the name of the field after a dot. Fields can only be accessed if they are in scope to the accessor. A struct can be called like a function to initialize it, it should be called with all of its fields in order.

A struct can also have methods. A method is a function defined inside the struct that affects its data. All methods can mutate the struct they are defined on. A method takes an implicit `self` parameter, which has the type of the struct. When a method is called on a reciver struct, it is implicitly passed to the method.

## Future

- Methods
- Initializers