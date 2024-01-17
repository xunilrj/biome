use crate::assert_semantics;

// Literals
assert_semantics! {
    ok_literal_number, "42/*TYPE number*/",
}

// Declarations
assert_semantics! {
    ok_type_check_fn_declaration, "function/*TYPE () -> number */ f() { return 1; }",
    ok_type_check_fn_declaration_two_returns, "function/*TYPE () -> number */ f() { if (true) { return 1; } else { return 2; } }",
    ok_type_check_fn_declaration_add_two_arguments, "function/*TYPE (number, number) -> number */ f(a: number, b: number) { return a + b; }",
}

// Expressions
assert_semantics! {
    ok_expression, "(1 + 1)/*TYPE number*/",
}