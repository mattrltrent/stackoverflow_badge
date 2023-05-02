pub fn format_number(num: i64) -> String {
    let num_str = num.to_string();
    let len = num_str.len();
    let mut res = String::new();

    for (i, c) in num_str.chars().enumerate() {
        let j = len - i - 1;
        if j % 3 == 2 && j != len - 1 {
            res.push(',');
        }
        res.push(c);
    }

    res
}
