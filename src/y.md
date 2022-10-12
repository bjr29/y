## Instructions
1. `VAR [name] [value];` Creates a variable with the name and value.
2. `ADD [name] [value];` Adds to the variable with the name by the value.
3. `PRINT [value];` Prints the byte array value as UTF8.
4. `PRINT_CHAR [value];` Prints the char value as UTF8.
5. `INPUT [name];` Get the inputted value as a byte from UTF8.
6. `FUNC [name] [return type] [args]?;` Creates a function with the name, arguments and return type.
7. `FUNC_END;` Closes the function's scope.
8. `RETURN [value]?;` Returns the value for the function.
9. `CALL [name] [args]?;` Returns the value for the function.
10. `IF [value] [comparison operator] [value];` Skips the code until reaches `END_IF`.
11. `ELSE_IF [value] [comparison operator] [value];` An extra branch of the previous `IF` statement.
12. `ELSE;` The default branch of `IF`.
13. `END_IF;` Closes the `IF` body.
14. `ARR_APPEND [name] [value] [index]?;` Adds value to the array with the name at the index.
15. `ARR_REMOVE [name] [index];` Removes the value from the array at the index.
16. `ARR_GET [name] [index] [name];` Gets the value from the array at the index and stores it in the variable.
17. `FILE_READ [value] [name];` Reads the file content byte by byte.
18. `FILE_WRITE [value] [name] [value];` Writes the file content byte by byte.

## Keys

## Symbols
- `#` Byte
- `%` Float
- `!` Null
