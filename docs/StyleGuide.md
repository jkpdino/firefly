# Firefly Style Guide

This document defines the recommended coding style and conventions for the Firefly programming language.

## General Principles

- **Consistency**: Follow established patterns within the codebase
- **Readability**: Write code that is easy to read and understand
- **Simplicity**: Prefer clear, straightforward solutions over clever ones
- **Expressiveness**: Use meaningful names that communicate intent

## Naming Conventions

### Capitalization

- **Modules**: `PascalCase` with dots for hierarchy (e.g., `Test.Factorial`, `User.Management`, `File.System`)
- **Types** (structs, enums): `PascalCase` for public, `snake_case` for internal (e.g., `Person`, `HttpClient` vs `internal_type`)
- **Functions**: `camelCase` for public, `snake_case` for internal (e.g., `calculateTotal`, `sayHi` vs `internal_helper`)
- **Variables**: `camelCase` (e.g., `userName`, `totalCount`, `usersLoggedIn`)
- **Constants**: `SCREAMING_SNAKE_CASE` (e.g., `MAX_RETRIES`, `DEFAULT_TIMEOUT`)
- **Initializers**: Always use `init` (lowercase)

### Meaningful Names

```firefly
// Good
func calculateMonthlyPayment(principal: float, rate: float, months: int) -> float

// Bad
func calc(p: float, r: float, m: int) -> float
```

### Abbreviations

Avoid abbreviations unless they are widely understood:

```firefly
// Good
var userCount: int;
var httpClient: HttpClient;

// Bad
var usrCnt: int;
var hc: HttpClient;
```

## Code Organization

### Module Structure

```firefly
module Com.Example.UserManagement;

// Imports first
import Std.Collections.List;
import Std.Io.Print;

// Public structs and functions
public struct User {
    // ...
}

public func createUser(name: string) -> User {
    // ...
}

// Private helpers at the end (use snake_case for internal functions)
func validate_user_name(name: string) -> bool {
    // ...
}
```

### Function Organization

- Parameter labels should be descriptive and aid readability
- Use parameter labels when the purpose isn't obvious from context

```firefly
// Good
func transfer(amount: float, from: Account, to: Account)
func createRectangle(width: float, height: float)

// Consider labels for clarity
func calculateCompoundInterest(principal: float, rate: float, periods: int)
func calculateCompoundInterest(principal amount: float, annualRate: float, compoundingPeriods: int)
```

## Formatting

### Indentation

- Use 2 spaces for indentation (no tabs)
- Align continuation lines with the opening delimiter

```firefly
func complexCalculation(
  firstParameter: float,
  secondParameter: float,
  thirdParameter: float
) -> float {
  return firstParameter * secondParameter + thirdParameter;
}
```

### Line Length

- Target 80 characters per line
- Break long lines at logical points
- Prefer breaking before operators

```firefly
// Good
var result = veryLongFunctionName(parameterOne, parameterTwo)
  + anotherLongCalculation(parameterThree, parameterFour);

// Avoid
var result = veryLongFunctionName(parameterOne, parameterTwo) + anotherLongCalculation(parameterThree, parameterFour);
```

### Spacing

```firefly
// Good spacing
func calculateArea(width: float, height: float) -> float {
  var area = width * height;
  
  if area > 100.0 {
    print("Large area detected");
  }
  
  return area;
}
```

### Braces

- Opening brace on the same line
- Closing brace on its own line, aligned with the statement that opened it

```firefly
if condition {
  doSomething();
} else if otherCondition {
  doSomethingElse();
} else {
  doDefault();
}
```

## Type Annotations

### When to Include Types

- **Always required**: Function parameters and return types
- **Optional but recommended**: Variables when type isn't obvious from context
- **Optional**: Local variables with obvious types (e.g., literals)

```firefly
// Required
func processData(input: string) -> List<string> {
  // Obvious type - annotation optional
  var items = List<string>();
  
  // Type not obvious - recommend annotation
  var result: ProcessResult = complex_processing(input);
  
  return items;
}
```

## Struct Design

### Field Organization

```firefly
public struct User {
  // Public fields first
  public var name: string;
  public var email: string;
  
  // Private fields after
  var id: int;
  var createdAt: DateTime;
  
  // Initializers
  init(name: string, email: string) {
    self.name = name;
    self.email = email;
    self.id = generate_user_id();
    self.createdAt = DateTime.now();
  }
  
  // Public methods
  public func displayInfo() {
    print(self.name + " (" + self.email + ")");
  }
  
  // Private methods last (use snake_case for internal functions)
  func validate_email() -> bool {
    // validation logic
  }
}
```

### Initializer Guidelines

- Use descriptive parameter labels when parameters don't match field names
- Initialize all fields, even if with default values
- Perform validation in initializers when appropriate

```firefly
struct Rectangle {
  var width: float;
  var height: float;
  var area: float;
  
  init(width: float, height: float) {
    if width <= 0.0 || height <= 0.0 {
      // Handle invalid input
    }
    
    self.width = width;
    self.height = height;
    self.area = width * height;
  }
}
```

## Error Handling

### Validation

- Validate inputs early and clearly
- Use descriptive error messages
- Handle edge cases explicitly

```firefly
func divide(numerator: float, denominator: float) -> float {
  if denominator == 0.0 {
    // Proper error handling would go here
    print("Error: Division by zero");
    return 0.0;
  }
  
  return numerator / denominator;
}
```

## Comments and Documentation

### When to Comment

- Complex algorithms or business logic
- Non-obvious decisions or workarounds
- Public API documentation

### Comment Style

```firefly
// Single-line comments for brief explanations

/*
 * Multi-line comments for longer descriptions
 * or detailed explanations of complex logic
 */

public func complex_algorithm(input: List<int>) -> int {
  // This implements the Foo-Bar algorithm from Smith et al. (2023)
  // We use a modified approach to handle edge case X
  
  /*
   * The main loop processes elements in three phases:
   * 1. Validation and normalization
   * 2. Core computation
   * 3. Result aggregation
   */
  
  // Implementation details...
}
```

## Import Organization

### Import Ordering

1. Standard library imports
2. Third-party library imports
3. Local module imports

```firefly
// Standard library
import Std.Collections.List;
import Std.Io.Print;

// Third-party (if any)
import External.Math.Statistics;

// Local modules
import Com.Example.Utils.Validation;
import Com.Example.Models.User;
```

## Performance Considerations

### Memory Management

- Prefer stack allocation when possible
- Be mindful of object creation in loops
- Reuse objects when appropriate

```firefly
// Good - reuse object
var buffer = StringBuilder();
for item in items {
  buffer.clear();
  buffer.append(processItem(item));
  results.add(buffer.toString());
}

// Less efficient - creates new objects each iteration
for item in items {
  var processed = processItem(item);
  results.add(processed);
}
```

## Testing Conventions

### Test Organization

- Test files should mirror source file organization
- Use descriptive test names that explain what is being tested

```firefly
module Tests.UserManagement;

import Com.Example.UserManagement.User;

func testUserCreationWithValidData() {
  var user = User("John Doe", "john@example.com");
  
  // Assertions would go here
}

func testUserCreationWithInvalidEmail() {
  // Test error handling
}
```

## Version-Specific Guidelines

### Version 0 Profile 2 Features

When using current language features:

- Leverage local type inference for cleaner code
- Use parameter labels for better API design  
- Implement proper struct initializers
- Take advantage of method overloading when appropriate

## Tools and Automation

### Recommended Tools

- Use `cargo fmt` equivalent when available for consistent formatting
- Use `cargo clippy` equivalent for style and correctness checks
- Set up editor integration for real-time feedback

---

This style guide is a living document and will evolve as the Firefly language develops. When in doubt, prioritize consistency with existing code and readability for future maintainers.