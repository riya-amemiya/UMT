use umt_rust::math::umt_math_converter;

#[test]
fn test_math_converter_multiplication() {
    assert_eq!(
        umt_math_converter("1250*1250"),
        "1500*1000+400*100+200*100+50*50"
    );
    assert_eq!(
        umt_math_converter("1350*1350"),
        "1700*1000+600*100+400*100+200*100+50*50"
    );
    assert_eq!(
        umt_math_converter("1550*1550"),
        "2100*1000+1000*100+800*100+600*100+400*100+200*100+50*50"
    );
}

#[test]
fn test_math_converter_exponentiation() {
    assert_eq!(
        umt_math_converter("1250^2"),
        "1500*1000+400*100+200*100+50*50"
    );
    assert_eq!(
        umt_math_converter("1350^2"),
        "1700*1000+600*100+400*100+200*100+50*50"
    );
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

#[test]
fn test_math_converter_empty() {
    assert_eq!(umt_math_converter(""), "");
}

#[test]
fn test_math_converter_different_operands() {
    assert_eq!(umt_math_converter("100*200"), "100*200");
    assert_eq!(umt_math_converter("1250*1350"), "1250*1350");
}

#[test]
fn test_math_converter_small_square() {
    assert_eq!(umt_math_converter("2*2"), "2*2+0*0");
}

#[test]
fn test_math_converter_stops_once_expansion_repeats() {
    assert_eq!(umt_math_converter("5*5"), "5*5+0*0");
    assert_eq!(umt_math_converter("10*10"), "10*10+0*0");
    assert_eq!(umt_math_converter("100*100"), "100*100+0*0");
    assert_eq!(umt_math_converter("10^3"), "10*10+0*0");
    assert_eq!(umt_math_converter("2000*2000"), "3000*1000+1000*1000+0*0");
}
