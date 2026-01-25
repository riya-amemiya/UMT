use umt_rust::math::umt_math_converter;

#[test]
fn test_math_converter_multiplication() {
    assert_eq!(umt_math_converter("1250*1250"), "1500*1000+400*100+200*100+50*50");
    assert_eq!(umt_math_converter("1350*1350"), "1700*1000+600*100+400*100+200*100+50*50");
    assert_eq!(umt_math_converter("1550*1550"), "2100*1000+1000*100+800*100+600*100+400*100+200*100+50*50");
}

#[test]
fn test_math_converter_exponentiation() {
    assert_eq!(umt_math_converter("1250^2"), "1500*1000+400*100+200*100+50*50");
    assert_eq!(umt_math_converter("1350^2"), "1700*1000+600*100+400*100+200*100+50*50");
}

#[test]
fn test_math_converter_no_conversion() {
    assert_eq!(umt_math_converter("1250"), "1250");
}

#[test]
fn test_math_converter_addition() {
    assert_eq!(umt_math_converter("1250+1250"), "1250+1250");
}

#[test]
fn test_math_converter_invalid() {
    assert_eq!(umt_math_converter("abc"), "abc");
}

#[test]
fn test_math_converter_zero() {
    assert_eq!(umt_math_converter("0*0"), "0*0");
}
