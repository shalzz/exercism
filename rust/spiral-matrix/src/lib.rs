pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let index = size as usize;
    let mut res = vec![vec![0; index]; index];
    let mut count = 1;

    let mut row = index;
    let mut col = index;
    let mut row_s = 0;
    let mut col_s = 0;

    for i in 0..row {
        // traverse the first row
        for j in col_s..col {
            res[i][j] = count;
            count += 1;
        }
        row_s += 1;

        // traverse the last column
        for j in row_s..row {
            res[j][col - 1] = count;
            count += 1;
        }
        col -= 1;

        // traverse the last row
        for j in (col_s..col).rev() {
            res[row - 1][j] = count;
            count += 1;
        }
        row -= 1;

        // traverse the first column
        for j in (row_s..row).rev() {
            res[j][col_s] = count;
            count += 1;
        }
        col_s += 1;
    }
    res
}
