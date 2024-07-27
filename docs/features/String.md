# String

## Description

A string represents text data using the UTF-8 encoding. Quoted string literals must be on one line and can't contain special characters, while multi-line string literals can span multiple lines and can contain anything. Raw string literals have escape sequences treated as text.

## Syntax

Quoted String Literals

- Starts with a double quote
- Ends with a double quote
- Can't contain new lines
- Escape sequences
    - \\
    - \"
    - \'
    - \0
    - \a
    - \b
    - \e
    - \f
    - \n
    - \r
    - \t
    - \v
    - \xXXXX

Multiline String Literals

- Starts with at least 3 double quotes
- Ends with the same number of double quotes
- Removes beginning and ending newlines
- Unindents a string to the level of the closing quotes
- Contains string literals

Raw String Literals

Raw string literals don't consider escape sequences. They are marked by the prefix raw.


## Future

- Postfixes and prefixes
- Interpolation