pub fn print_parens(
    path: String,
    open_used: i32,
    close_used: i32,
    n: i32,
    output_vec: &mut Vec<String>,
) {
    if open_used == n && close_used == n {
        output_vec.push(path.clone());
        return;
    }
    if open_used < n {
        print_parens(
            format!("{}{}", path, "("),
            open_used + 1,
            close_used,
            n,
            output_vec.as_mut(),
        );
    }
    if close_used < open_used {
        print_parens(
            format!("{}{}", path, ")"),
            open_used,
            close_used + 1,
            n,
            output_vec.as_mut(),
        );
    }
}

pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut output_vec: Vec<String> = vec![];
    let path = "";
    print_parens(path.to_string(), 0, 0, n, output_vec.as_mut());
    return output_vec;
}
