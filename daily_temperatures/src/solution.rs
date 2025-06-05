pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut output: Vec<i32> = vec![0; temperatures.len()];
    let mut stack: Vec<(i32, usize)> = Vec::new();

    for (index, temp) in temperatures.iter().enumerate() {
        let temp = *temp;
        while !stack.is_empty() && temp > stack.last().unwrap().0 {
            let stack_temp = stack.pop().unwrap();
            output[stack_temp.1] = (index - stack_temp.1) as i32;
        }
        stack.push((temp, index));
    }
    return output;
}
