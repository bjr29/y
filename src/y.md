## Instructions
1. `VAR [name] [value];` Creates a variable with the name and value.
2. `ADD [name] [value];` Adds to the variable with the name by the value.
3. `PRINT [value];` Prints the value as UTF-8.
5. `INPUT [name];` Get the inputted value as a byte array from UTF8.
6. `FUNC [return type] [name] [args... (value type, name)]?;` Creates a function with the name, arguments and return type.
7. `FUNC_END;` Closes the function's scope.
8. `RETURN [value]?;` Returns the value for the function.
9. `CALL [name] [var] [args...]?;` Returns the value into var for the function.
10. `IF [value] [comparison operator] [value];` Skips the code until reaches `END_IF`.
11. `ELSE_IF [value] [comparison operator] [value];` An extra branch of the previous `IF` statement.
12. `ELSE;` The default branch of `IF`.
13. `END_IF;` Closes the `IF` body.
14. `ARR_VAR [name] [array...]` Creates an array with the variable name, the array is passed like regular arguments.
15. `ARR_APPEND [name] [value] [index]?;` Adds value to the array with the name at the index.
16. `ARR_REMOVE [name] [index];` Removes the value from the array at the index.
17. `ARR_GET [name] [index] [name];` Gets the value from the array at the index and stores it in the variable.
18. `ARR_LEN [name] [name];` Gets the length of the array (1st name) places it into a variable (2nd name).
19. `FILE_READ [value] [name];` Reads the file content byte by byte.
20. `FILE_WRITE [value] [name] [value];` Writes the file content byte by byte.
21. `WHILE [value] [comparison operator] [value];` Loops until the condition is false.
22. `WHILE_END;` Closes the while loop.
23. `BREAK;` Used to break a while loop.
24. `IMPORT [name];` Import functions from another file.

## Symbols
- `#` Byte
- `%` Float
- `!` Null
