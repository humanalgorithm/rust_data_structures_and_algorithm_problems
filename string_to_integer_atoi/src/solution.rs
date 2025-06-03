pub fn my_atoi(s: String) -> i32 {
    let str_chars: Vec<char> = s.chars().collect();
    let str_chars_len = str_chars.len();
    let (mut index_start, mut output_str) = (0, String::new());

    while index_start < str_chars_len {
        if str_chars[index_start] == '_' || str_chars[index_start] == ' ' {
            index_start += 1;
            continue;
        }
        if str_chars[index_start] == '-' {
            output_str.push_str(&format!("{}", str_chars[index_start]));
            index_start += 1;
            break;
        } else if str_chars[index_start] == '+' {
            index_start += 1;
            break;
        }
        if !str_chars[index_start].is_digit(10) {
            return 0;
        } else {
            break;
        }
    }

    for x in index_start..str_chars_len {
        if !(str_chars[x].is_digit(10)) {
            break;
        }
        output_str.push_str(&format!("{}", str_chars[x]));
    }

    let output_num: i32 = match output_str.parse() {
        Ok(num) => num,
        Err(err) => match err.kind() {
            std::num::IntErrorKind::NegOverflow => i32::pow(2, 31),
            std::num::IntErrorKind::PosOverflow => i32::pow(2, 31) - 1,
            std::num::IntErrorKind::InvalidDigit => 0,
            _ => 0,
        },
    };
    return output_num;
}
