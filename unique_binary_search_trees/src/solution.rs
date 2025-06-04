pub fn num_trees(n: i32) -> i32 {
    let vec_size: usize = n as usize + 1;
    let mut num_trees = vec![1; vec_size];

    for nodes in 2..n + 1 {
        let mut total = 0;
        for root in 1..nodes + 1 {
            let left = root - 1;
            let right = nodes - root;
            total += num_trees[left as usize] * num_trees[right as usize];
        }
        num_trees[nodes as usize] = total;
    }
    return num_trees[n as usize];
}
