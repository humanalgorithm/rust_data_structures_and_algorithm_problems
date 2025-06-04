pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut res: Vec<i32> = vec![];
    let (mut carry, mut add) = (0, 1);
    for x in (0..digits.len()).rev() {
        add = add + digits[x] + carry;
        if add > 9 {
            carry = 1;
            add = 0;
        } else {
            carry = 0;
        }
        res.push(add);
        add = 0;
    }
    if carry == 1 {
        res.push(1)
    };
    return res.iter().rev().map(|m| *m).collect();
}
