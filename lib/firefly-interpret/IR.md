# Firefly IR

## Types

## Globals

## Functions

## Locals

A local is a value used by a function, including its parameters. Each local is referenced using a percent sign before its index within a function. A local is declared like this:

```
local %num: ty
```

## Places

A place is an area in memory that holds a value of a certain type. A place can be a local, a global, or any operation performed on one. A place can have an undefined value, but it can't be used while it has an undefined value. A place can be assigned an immediate, and an immediate can be fetched through an operation on a place.

Defined places include:

- Local
- Global
- Tuple Index
- Struct Field

## Immediates

An immediate is a value that doesn't necessarily represent a value in memory. Different immediates include:

- Constant
- Functions
- Function Calls
- Copy from place
- Intrinsic

## Assignments

A basic instruction consists of assigning the value of an immediate to a place. This is denoted by:

```
place := immediate
```

## Basic Blocks

A basic block represents a piece of code that can be jumped to. Each basic block has exactly one terminator.

## Terminators

A terminator is a piece of code that leaves a basic block. Terminators include:

- Branch
- Branch if
- Call
- Return
- Panic

## Factorial

```

def factorial(%0) {
    local %0: int
    local %1: bool
    local %2: int
    local %3: int

bb0:
    %1 := compare[less_than_or_equal] (move %0, constant 1)

    branch if move %1 (@bb1 else @bb2)

bb1:
    return 1

bb2:
    %2 := sub (move %0, constant 1)
    %3 := mul (move %0, move %2)

    return (%3)
}

```