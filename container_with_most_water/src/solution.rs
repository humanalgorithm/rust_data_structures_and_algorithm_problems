use std::cmp;

pub fn max_area(height: Vec<i32>) -> i32 {
    let (mut max_vol, heights_len) = (0, height.len());
    let (mut i, mut j) = (0, heights_len - 1);

    while i < j && i < heights_len {
        let ci_height = height[i as usize] as i32;
        let cj_height = height[j as usize] as i32;

        let curr_vol: i32 = cmp::min(ci_height as i32, cj_height) * (j as i32 - i as i32);
        max_vol = cmp::max(max_vol, curr_vol);
        if cj_height > ci_height {
            i += 1;
        } else {
            j -= 1;
        }
    }
    return max_vol;
}
