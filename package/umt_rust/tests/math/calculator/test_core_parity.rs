use umt_rust::math::calculator::umt_calculator_core;

#[test]
fn test_core_repeated_calls_reuse_cached_patterns() {
    for _ in 0..3 {
        assert_eq!(umt_calculator_core("(2+3)*4", None), "20");
        assert_eq!(umt_calculator_core("8/2/2", None), "2");
        assert_eq!(umt_calculator_core("1-2-3", None), "-4");
        assert_eq!(umt_calculator_core("2^3", None), "8");
        assert_eq!(umt_calculator_core("(5)", None), "5");
    }
}

#[test]
fn test_core_unresolvable_expressions_are_returned_unchanged() {
    assert_eq!(umt_calculator_core("(a)", None), "(a)");
    assert_eq!(umt_calculator_core("a*b", None), "a*b");
    assert_eq!(umt_calculator_core("a/b", None), "a/b");
    assert_eq!(umt_calculator_core("a+b", None), "a+b");
}
