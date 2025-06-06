# Data Structures and Algorithm Problems in Rust

# File Structure

Each folder name matches the name of a corresponding problem. 
Each problem can be found at `https://leetcode.com/problems/{problem}`

The following files are contained in each folder:  

`solution.rs`: Solution code for solving the data structure or algorithm problem  
`data.rs`: Data setup for testing fixtures  
`execute.rs`: Sets up the data and calls solution code with data as parameters  
`description.html`: Description of the given problem.  


# Run Instructions

Solutions can be run from Cargo using the following syntax: 

```console
cargo run -p {problem_name}
```

Example: 
```console
cargo run -p validate_binary_search_tree

Running Case 1...
Input data:
root: Some(RefCell { value: TreeNode { val: 2, left: Some(RefCell { value: TreeNode { val: 1, left: None, right: None } }), right: Some(RefCell { value: TreeNode { val: 3, left: None, right: None } }) } })
n: Some(RefCell { value: TreeNode { val: 2, left: Some(RefCell { value: TreeNode { val: 1, left: None, right: None } }), right: Some(RefCell { value: TreeNode { val: 3, left: None, right: None } }) } })

Result:
true

Running Case 2...
Input data:
root: Some(RefCell { value: TreeNode { val: 5, left: Some(RefCell { value: TreeNode { val: 1, left: None, right: None } }), right: Some(RefCell { value: TreeNode { val: 4, left: Some(RefCell { value: TreeNode { val: 3, left: None, right: None } }), right: Some(RefCell { value: TreeNode { val: 6, left: None, right: None } }) } }) } })
n: Some(RefCell { value: TreeNode { val: 5, left: Some(RefCell { value: TreeNode { val: 1, left: None, right: None } }), right: Some(RefCell { value: TreeNode { val: 4, left: Some(RefCell { value: TreeNode { val: 3, left: None, right: None } }), right: Some(RefCell { value: TreeNode { val: 6, left: None, right: None } }) } }) } })

Result:
false
```
