# Version 0 Profile 2

## Highlight Features

- [x] Local variable inference
  - [x] Change parse to make type optional
  - [x] Copy the type from the value if needed
- [x] Operators
  - [x] Make a list of allowed operators
  - [x] Parse operators with precedence
  - [x] Allow methods for intrinsics
  - [x] Replace operators with method calls
  - [x] Test method call operators
  - [x] Add string operators
  - [ ] Add errors for operators (ish)
- [x] Parameter Labels
  - [x] Parse parameter labels
  - [x] Add parameters to signatures
  - [x] Figure out whether to use = or :
  - [x] Parse argument labels
- [x] Function overloading
  - [x] Error for more than one
  - [x] Select overloads from a path
  - [x] Select overloads in scope
- [x] Struct Initializers
  - [x] Add parsing for initializers
  - [x] Add static delegate component
  - [x] Add AST lowering for initializers
  - [x] Add HIR lowering for initializers
  - [x] Add MIR generation with proper struct initialization
  - [x] Add basic test case
  - [ ] Add privacy checks for static delegate
  - [ ] Add overload for initializers
- [x] Style guide
  - [x] Capitalization
  - [x] Naming conventions
  - [x] Code organization
  - [x] Formatting rules
  - [x] Struct design guidelines
  - [x] Import organization
