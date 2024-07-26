# Import

## Overview

An import statement imports symbols into its parent. It can import everything from a namespace, or specific symbols. It can also rename a symbol. An import statement cannot reexport a symbol.

## Syntax

The basic syntax of an import statement is the `import` keyword followed by a path.

```
import path.to.module
```

Specific symbols from a namespace can also be imported. This can be done by providing a list of the symbols after the import.

```
import path.to.module (Symbol1, Symbol2)
```

Symbols can also be renamed with the `as` keyword. Either the module or symbol can be renamed like this. The syntax looks like this:

```
import path.to.module as ModuleName (Symbol as SymbolName)
```

## Semantics

- If only a path is provided, every visible symbol in that module will be added to the current scope
- If a list of symbols is provided, only those are imported
- If the original module is renamed, then a symbol for it is created and only the explicitly included symbols will be included in scope
- If the path of an import statement is not an existing module, an error will be thrown
- If a symbol is provided that doesn't exist, an error will be thrown
- If a symbol is provided that isn't visible, an error will be thrown
- If a symbol is provided that shadows another symbol, an error will be thrown

## Future