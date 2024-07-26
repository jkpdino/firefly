# Visibility

## Rules

Public symbols are visible in the root
Internal symbols are visible in the base module they are defined in
Fileprivate symbols are visible in the file they are defined in
Private symbols are visible in the immediate parent they are defined in

## Syntax

A visibility is optionally placed before an item. Each visibility has an associated keyword. When no visibility is specified, the Internal visibility is used.

```
public func foo() { }
internal func foo() { }
fileprivate func foo() { }
private func foo() { }
```

## How they work

Visibilities work by calculating a VisibleWithin component. This component calculates the most general entity a symbol is defined on. Whether a symbol is visible from a namespace can be checked by making a list of the ancestors of the namespace, and checking if the symbols VisibleWithin is in that list.

Visibility must be checked when adding a symbol to a SymbolTable, importing a symbol, when finding a static member, and when finding an instance member.