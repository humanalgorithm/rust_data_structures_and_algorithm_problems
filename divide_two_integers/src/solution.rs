pub fn divide(dividend: i32, divisor: i32) -> i32 {
    if dividend == i32::MIN && divisor == -1 {
        return i32::MAX;
    }
    if dividend == i32::MIN && divisor == 1 {
        return i32::MIN;
    }
    if dividend == i32::MIN && divisor == i32::MIN {
        return 1;
    }

    let sign = dividend.is_negative() && divisor.is_positive()
        || dividend.is_positive() && divisor.is_negative();

    let dividend = dividend.unsigned_abs();
    let divisor = divisor.unsigned_abs();

    let mut current_num = 0;
    let mut add_count = 0;

    if divisor == 1 {
        let dividend_i32 = dividend as i32;
        if sign {
            return -dividend_i32;
        } else {
            return dividend_i32;
        }
    }

    let log2_32 = (32 - 1) - divisor.leading_zeros();
    if 2_u32.pow(log2_32) == divisor && divisor < dividend {
        let dividend_i32 = dividend as i32;
        if sign {
            return dividend_i32 >> log2_32;
        } else {
            return dividend_i32 >> log2_32;
        }
    }

    loop {
        if current_num == dividend {
            break;
        }
        if current_num < dividend {
            current_num += divisor;
            add_count += 1;
        } else if current_num > dividend {
            add_count -= 1;
            break;
        }
    }
    if sign { -add_count } else { add_count }
}
