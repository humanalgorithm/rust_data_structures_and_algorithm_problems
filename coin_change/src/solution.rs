use std::cmp;

pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let mut dp: Vec<i32> = vec![amount + 1; (amount + 1) as usize];
    dp[0] = 0;
    for a in 1..(amount + 1) as usize {
        for c in coins.iter().map(|m| *m as usize) {
            if (a as i32 - c as i32) as i32 >= 0 {
                dp[a] = cmp::min(dp[a], 1 + dp[a - c])
            }
        }
    }
    return if dp[amount as usize] != amount + 1 {
        dp[amount as usize]
    } else {
        -1
    };
}
