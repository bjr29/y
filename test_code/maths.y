FUNC add a b;
    ADD a b;

    RETURN a;
FUNC_END;

FUNC mult a b;
    VAR increment a;
    VAR a 0;

    WHILE b > 0;
        ADD a increment;
    WHILE_END;

    RETURN a;
FUNC_END;

FUNC sub a b;

FUNC_END;

VAR add_result !;
CALL add add_result 15 15;
PRINT add_result;

VAR mult_result !;
CALL mult mult_result 10 3;
PRINT mult_result;

VAR sub_result !;
CALL sub sub_result 10 5;
PRINT sub_result;
