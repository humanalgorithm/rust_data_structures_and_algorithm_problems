pub fn reverse(x: i32) -> i32 {
    let (mut output_num, mut current) = (0, x);
    let int_len = match x.abs().checked_ilog10() {
        Some(value) => value + 1,
        None => return 0,
    };
    let mut ten_p = int_len - 1;

    while current != 0 {
        let (digit, multiple) = (current % 10, i32::pow(10, ten_p));
        match digit.checked_mul(multiple) {
            None => return 0,
            Some(result) => match result.checked_add(output_num) {
                None => return 0,
                Some(ok) => {
                    output_num = ok;
                }
            },
        };
        current = current / 10;
        ten_p = if ten_p != 0 { ten_p - 1 } else { 0 as u32 };
    }
    return output_num;
}
