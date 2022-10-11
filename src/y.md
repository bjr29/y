## Instructions
1. `VAR [name] [value];` Creates a variable with the name and value.
2. `ADD [name] [value];` Adds to the variable with the name by the value.
3. `PRINT [value];` Prints the value as ASCII.
4. `INPUT [name];` Get the inputted value as a byte from ASCII.
5. `FUNC [name] [return type] [args]?;` Creates a function with the name, arguments and return type.
6. `FUNC_END;` Closes the function's scope.
7. `RETURN [value]?;` Returns the value for the function.
8. `CALL [name] [args]?;` Returns the value for the function.
9. `IF [value] [comparison operator] [value];` Skips the code until reaches `END_IF`.
10. `ELSE_IF [value] [comparison operator] [value];` An extra branch of the previous `IF` statement.
11. `ELSE;` The default branch of `IF`.
12. `END_IF;` Closes the `IF` body.
13. `ARR_APPEND [name] [value] [index]?;` Adds value to the array with the name at the index.
14. `ARR_REMOVE [name] [index];` Removes the value from the array at the index.
15. `ARR_GET [name] [index] [name];` Gets the value from the array at the index and stores it in the variable.
16. `FILE_READ [value] [name];` Reads the file content byte by byte.
17. `FILE_WRITE [value] [name] [value];` Writes the file content byte by byte.

## Keys

## Symbols
- `#` Byte
- `%` Float
- `!` Null
