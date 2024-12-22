pub fn umt_math_separator(x: i32) -> Vec<i32> {
    let n: i32 = (x.to_string().len() - 1).try_into().unwrap();
    if n > 0 {
        return vec![
            10_i32.pow(n.try_into().unwrap()),
            (x - (10_i32.pow(n.try_into().unwrap()))),
        ];
    }
    vec![0, x]
}
