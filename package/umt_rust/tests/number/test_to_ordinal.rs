use umt_rust::number::umt_to_ordinal;

#[test]
fn test_st() {
    assert_eq!(umt_to_ordinal(1.0), "1st");
    assert_eq!(umt_to_ordinal(21.0), "21st");
    assert_eq!(umt_to_ordinal(31.0), "31st");
    assert_eq!(umt_to_ordinal(101.0), "101st");
    assert_eq!(umt_to_ordinal(121.0), "121st");
}

#[test]
fn test_nd() {
    assert_eq!(umt_to_ordinal(2.0), "2nd");
    assert_eq!(umt_to_ordinal(22.0), "22nd");
    assert_eq!(umt_to_ordinal(32.0), "32nd");
    assert_eq!(umt_to_ordinal(102.0), "102nd");
    assert_eq!(umt_to_ordinal(122.0), "122nd");
}

#[test]
fn test_rd() {
    assert_eq!(umt_to_ordinal(3.0), "3rd");
    assert_eq!(umt_to_ordinal(23.0), "23rd");
    assert_eq!(umt_to_ordinal(33.0), "33rd");
    assert_eq!(umt_to_ordinal(103.0), "103rd");
    assert_eq!(umt_to_ordinal(123.0), "123rd");
}

#[test]
fn test_th_special() {
    assert_eq!(umt_to_ordinal(11.0), "11th");
    assert_eq!(umt_to_ordinal(12.0), "12th");
    assert_eq!(umt_to_ordinal(13.0), "13th");
}

#[test]
fn test_th_special_hundreds() {
    assert_eq!(umt_to_ordinal(111.0), "111th");
    assert_eq!(umt_to_ordinal(112.0), "112th");
    assert_eq!(umt_to_ordinal(113.0), "113th");
}

#[test]
fn test_th_others() {
    assert_eq!(umt_to_ordinal(4.0), "4th");
    assert_eq!(umt_to_ordinal(5.0), "5th");
    assert_eq!(umt_to_ordinal(6.0), "6th");
    assert_eq!(umt_to_ordinal(7.0), "7th");
    assert_eq!(umt_to_ordinal(8.0), "8th");
    assert_eq!(umt_to_ordinal(9.0), "9th");
    assert_eq!(umt_to_ordinal(10.0), "10th");
    assert_eq!(umt_to_ordinal(20.0), "20th");
    assert_eq!(umt_to_ordinal(100.0), "100th");
}

#[test]
fn test_zero() {
    assert_eq!(umt_to_ordinal(0.0), "0th");
}
