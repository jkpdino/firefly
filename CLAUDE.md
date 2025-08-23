# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build and Run Commands

- **Build the project**: `cargo build`
- **Run the compiler**: `cargo run -- <files>` where `<files>` are the Firefly source files to compile
- **Run with HIR debug output**: `cargo run -- --print-hir <files>`
- **Run with MIR debug output**: `cargo run -- --print-mir <files>`
- **Run tests**: `cargo test`
- **Check code without building**: `cargo check`
- **Format code**: `cargo fmt`
- **Lint code**: `cargo clippy`

## Architecture Overview

Firefly is a multi-stage compiler with an interpreter backend, organized as a Rust workspace with the following compilation pipeline:

### Compilation Pipeline
1. **Parse** (`firefly-parser`) - Lexing and parsing Firefly source files using LALRPOP
2. **AST** (`firefly-ast`) - Abstract syntax tree representation
3. **AST Lower** (`firefly-ast-lower`) - Lowers AST to HIR with symbol resolution and type checking
4. **HIR** (`firefly-hir`) - High-level intermediate representation with resolved symbols and types
5. **HIR Lower** (`firefly-hir-lower`) - Transforms HIR to MIR
6. **MIR** (`firefly-mir`) - Mid-level intermediate representation for optimization and codegen
7. **Interpret** (`firefly-interpret`) - Execution engine that directly interprets MIR

### Key Components
- **Driver** (`firefly-driver`) - Main compilation orchestration with pipeline stages
- **Error System** (`firefly-errors`, `firefly-error-messages`) - Comprehensive error handling and reporting
- **Symbol System** (`firefly-mangle`) - Name mangling and symbol management
- **Span System** (`firefly-span`) - Source location tracking for error messages

### HIR Entity-Component System
HIR uses an entity-component architecture where:
- **Entities** represent language constructs (functions, structs, variables)
- **Components** add behavior/data (Symbol, Callable, HasType, HasValue, etc.)
- **Resolution** happens through symbol tables and member tables

## Language Features (Current Implementation)

The compiler currently supports "Version 0 Profile 2" features:
- Modules with explicit declarations
- Functions with parameter labels and overloading
- Structs with fields, methods, and initializers
- Local type inference (explicit types not required for local variables)
- Control flow: if/while/loops
- Operators and operator overloading
- Import system with symbol selection and renaming

## Test Structure

Tests are organized in `/tests/` by feature area:
- `Calling/` - Function and method invocation tests
- `Flow/` - Control flow constructs
- `Init/` - Initializer functionality (in development)
- `Structs/` - Struct definition and usage
- `Overloading/` - Function and method overloading

## Development Notes

- The project is actively implementing initializers (init functions) for structs
- Symbol resolution uses a two-phase approach: first resolve definitions, then resolve code
- Error messages include source spans and are designed to be user-friendly
- The interpreter directly executes MIR without generating machine code