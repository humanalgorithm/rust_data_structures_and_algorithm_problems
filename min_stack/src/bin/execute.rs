use min_stack::{data, solution};

fn case_1() {
    let data = data::data_1();
    println!("Input data:");
    println!("commands: {:?}", data.commands);
    println!("values: {:?} \n", data.values);

    let command = &data.commands[0];
    let input = &data.values[0];
    println!("command: {:?}, input: {:?}", command, input);
    let mut obj = solution::MinStack::new();
    println!("result: {:?}", obj);

    let command = &data.commands[1];
    let input = &data.values[1];
    println!("command: {:?}, input: {:?}", command, input);
    let res = obj.push(input[0]);
    println!("result: {:?}", res);

    let command = &data.commands[2];
    let input = &data.values[2];
    println!("command: {:?}, input: {:?}", command, input);
    let res = obj.push(input[0]);
    println!("result: {:?}", res);

    let command = &data.commands[3];
    let input = &data.values[3];
    println!("command: {:?}, input: {:?}", command, input);
    let res = obj.push(input[0]);
    println!("result: {:?}", res);

    let command = &data.commands[4];
    let input = &data.values[4];
    println!("command: {:?}, input: {:?}", command, input);
    let res = obj.get_min();
    println!("result: {:?}", res);

    let command = &data.commands[5];
    let input = &data.values[5];
    println!("command: {:?}, input: {:?}", command, input);
    let res = obj.pop();
    println!("result: {:?}", res);

    let command = &data.commands[6];
    let input = &data.values[6];
    println!("command: {:?}, input: {:?}", command, input);
    let res = obj.top();
    println!("result: {:?}", res);

    let command = &data.commands[7];
    let input = &data.values[7];
    println!("command: {:?}, input: {:?}", command, input);
    let res = obj.get_min();
    println!("result: {:?}\n", res);
}

fn main() {
    println!("Running Case 1...");
    case_1();
}
