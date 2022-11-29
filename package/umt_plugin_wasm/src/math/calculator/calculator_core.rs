pub fn umt_calculator_core(mut x: String) -> String {
    //符号反転
    x = x.replace("--", "+");
    x = x.replace("++", "+");
    x = x.replace("+-", "+0-");
    x = x.replace("-+", "+0-");
    x
}
