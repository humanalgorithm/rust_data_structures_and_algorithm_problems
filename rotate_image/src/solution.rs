pub fn rotate(matrix: &mut Vec<Vec<i32>>) -> &mut Vec<Vec<i32>> {
    let matrix_len = matrix.len();
    let matrix_end_index = matrix.len() - 1;
    let (mut layer, mut layer_len) = (0, matrix_len);

    while layer < (matrix_len) / 2 {
        let mut increment = 0;
        while increment < layer_len - 1 {
            let lu = (0 + layer, increment + layer);
            let ru = (increment + layer, matrix_end_index - layer);
            let rd = (
                matrix_end_index - layer,
                matrix_end_index - layer - increment,
            );
            let ld = (matrix_end_index - layer - increment, 0 + layer);

            let ru_t = matrix[ru.0][ru.1];
            let rd_t = matrix[rd.0][rd.1];
            let ld_t = matrix[ld.0][ld.1];
            let lu_t = matrix[lu.0][lu.1];

            matrix[lu.0][lu.1] = ld_t;
            matrix[ru.0][ru.1] = lu_t;
            matrix[rd.0][rd.1] = ru_t;
            matrix[ld.0][ld.1] = rd_t;

            increment += 1;
        }
        layer_len -= 2;
        layer += 1;
    }
    matrix
}
