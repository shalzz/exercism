pub struct PascalsTriangle {
    row_count: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle { row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        (0..self.row_count)
            .map(|row| (0..row + 1).map(|index| combination(row, index)).collect())
            .collect()
    }
}

/**
 * Prints out any value `k` in the nth row of a Pascal's triangle
 *
 * n: row
 * k: position in row
 */
fn combination(n: u32, k: u32) -> u32 {
    // nC0 and nCn are base cases
    // so we exit early with the known solution.
    if k == 0 || k == n {
        return 1;
    }

    // calculate the current constant
    // with our known values
    // and multiply with the lower combination
    combination(n, k - 1) * (n + 1 - k) / k
}
