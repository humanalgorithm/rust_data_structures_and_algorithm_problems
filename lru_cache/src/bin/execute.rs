use lru_cache::{data, solution};

fn case_1() {
    let data = data::data_1();
    println!("Input data:");
    println!("commands: {:?}", data.commands);
    println!("values: {:?} \n", data.values);

    let command = &data.commands[0];
    let input = &data.values[0];
    println!("command: {:?}, input: {:?}", command, input);
    let mut obj = solution::LRUCache::new(input[0]);
    println!("result: {:?}", obj);

    let command = &data.commands[1];
    let input = &data.values[1];
    println!("command: {:?}, input: {:?}", command, input);
    let res = obj.put(input[0], input[1]);
    println!("result: {:?}", res);

    let command = &data.commands[2];
    let input = &data.values[2];
    println!("command: {:?}, input: {:?}", command, input);
    let res = obj.put(input[0], input[1]);
    println!("result: {:?}", res);

    let command = &data.commands[3];
    let input = &data.values[3];
    println!("command: {:?}, input: {:?}", command, input);
    let res = obj.get(input[0]);
    println!("result: {:?}", res);

    let command = &data.commands[4];
    let input = &data.values[4];
    println!("command: {:?}, input: {:?}", command, input);
    let res = obj.put(input[0], input[1]);
    println!("result: {:?}", res);

    let command = &data.commands[5];
    let input = &data.values[5];
    println!("command: {:?}, input: {:?}", command, input);
    let res = obj.get(input[0]);
    println!("result: {:?}", res);

    let command = &data.commands[6];
    let input = &data.values[6];
    println!("command: {:?}, input: {:?}", command, input);
    let res = obj.put(input[0], input[1]);
    println!("result: {:?}", res);

    let command = &data.commands[7];
    let input = &data.values[7];
    println!("command: {:?}, input: {:?}", command, input);
    let res = obj.get(input[0]);
    println!("result: {:?}", res);

    let command = &data.commands[8];
    let input = &data.values[8];
    println!("command: {:?}, input: {:?}", command, input);
    let res = obj.get(input[0]);
    println!("result: {:?}", res);

    let command = &data.commands[9];
    let input = &data.values[9];
    println!("command: {:?}, input: {:?}", command, input);
    let res = obj.get(input[0]);
    println!("result: {:?}\n", res);
}

fn main() {
    println!("Running Case 1...");
    case_1();
}
