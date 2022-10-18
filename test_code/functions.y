FUNC test x;
    PRINT x;
    ADD x x;
    PRINT x;
    RETURN x;
FUNC_END;
CALL test a 10;
PRINT a;