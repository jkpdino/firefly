# Operators

An operator allows for convenient calling of a method. Operators use symbols to denote a method.

## Fixes

### Infix

Most operators are infix, or between 2 values. The syntax of an infix operator is `left` `*` `right`, where `*` is the operator.

### Prefix

A prefix operator operates on one value, and is placed before it.

### Postfix

A postfix operator operates on one value, and is placed after it. Postfix and infix operators cannot use the same symbol.

## Precedence

An expression such as `a + b * c` could be parse in one of two ways: `((a + b) * c)`, or `(a + (b * c))`. We disambigute between these using precedence. The operator with the higher precedence is the one that is in the innermost expression, with the left expression taking the tie. We use what is called a Pratt parser to do this. A Pratt parser makes decisions recursively on whether to shift or reduce for each expression. Take `a + b * c + d`, and we will go through the decision making process.

- Start with no binding power
- Find `a` as the left expression
- Find `+` as the operator
- The binding power of `+` is higher than no binding power, so we shift
- Parse the right expression
  - Find `b` as the left expression
  - Find `*` as the operator
  - The binding power of `*` is higher than `+`, so we shift
  - Parse the right expression
    - Find `c` as the left expression
    - Find `+` as the operator
    - The binding power of `+` is lower than `*`, so we reduce
  - The binding power of `+` is the same as `+`, so we reduce
- The binding power of `+` is greater than none, so we shift

The final expression is `(a + (b * c)) + d`.

### Precedence Groups

Groups are listed from highest to lowest precedence.

| Group          | Precedence | Operators                                    |
| -------------- | ---------- | -------------------------------------------- |
| Postfix        | 900        |                                              |
| Prefix         | 800        | `!` `+` `-`                                  |
| Exponentitive  | 700        | `<<` `>>`                                    |
| Multiplicative | 600        | `*` `/` `%` `&`                              |
| Additive       | 500        | `+` `-` `\|` `^`                             |
| Relational     | 400        | `<` `>` `<=` `>=` `==` `!=`                  |
| LogicalAnd     | 300        | `&&`                                         |
| LogicalOr      | 200        | `\|\|`                                       |
| Assignment     | 100        | `=` `+=` `-=` `*=` `/=` `%=` `&=` `\|=` `^=` |
| None           | 0          |                                              |

## Operators

### Prefix

#### Invert

`invert()`

#### Identity

`identity()`

#### Negate

`negate()`

## Exponentitive

#### Shift Left

`shiftLeft(by)`

#### Shift Right

`shiftRight(by)`

### Multiplicative

#### Multiply

`multiply(by)`

#### Divide

`divide(by)`

#### Modulo

`modulo(by)`

#### Bitwise And

`bitAnd(with)`

### Additive

#### Add

`add(amount)`

#### Subtract

`subtract(amount)`

#### Bitwise Or

`bitOr(with)`

#### Bitwise Xor

`bitXor(with)`

### Relational

#### Less Than

`lessThan(other)`

#### Less Than Or Equal

`lessThanOrEquals(other)`

#### Greater Than

`greaterThan(other)`

#### Greater Than Or Equal

`greaterThanOrEquals(other)`

#### Equals

`equals(other)`

#### Not Equals

`notEquals(other)`

### Logic

#### Logical And

`and(other)`

#### Logical Or

`or(other)`
