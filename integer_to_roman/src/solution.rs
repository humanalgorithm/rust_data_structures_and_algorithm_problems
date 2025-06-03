use std::collections::HashMap;

pub fn int_to_roman(num: i32) -> String {
    let mut output_str = String::new();
    let digits: Vec<_> = num
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect();
    let map = HashMap::from([
        (1, 'I'),
        (5, 'V'),
        (10, 'X'),
        (50, 'L'),
        (100, 'C'),
        (500, 'D'),
        (1000, 'M'),
    ]);
    let digit_len = digits.len();
    let mut exp = digit_len as u32;
    exp = exp - 1;
    let mut i = 0;
    loop {
        let curr_num = digits[i];
        let curr_key = 10_i32.pow(exp);
        let roman = map.get(&curr_key).unwrap();
        if curr_num < 5 && curr_num != 4 {
            for _ in 0..curr_num {
                output_str.push(*roman);
            }
        } else if curr_num == 4 {
            output_str.push(*roman);
            let next_key = curr_key * 5;
            let next_roman = map.get(&next_key).unwrap();
            output_str.push(*next_roman);
        } else if curr_num == 5 {
            let next_key = curr_key * 5;
            let next_roman = map.get(&next_key).unwrap();
            output_str.push(*next_roman);
        } else if curr_num > 5 && curr_num != 9 {
            let first_key = curr_key * 5;
            let first_roman = map.get(&first_key).unwrap();
            output_str.push(*first_roman);
            for _ in 5..curr_num {
                output_str.push(*roman);
            }
        } else if curr_num == 9 {
            let second_key = curr_key * 10;
            let second_roman = map.get(&second_key).unwrap();
            output_str.push(*roman);
            output_str.push(*second_roman);
        }
        if i == digit_len - 1 {
            break;
        }
        i += 1;
        exp -= 1;
    }
    return output_str;
}
