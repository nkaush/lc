struct Solution { }

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut grid: Vec<Vec<i32>> = vec![vec![-1; n as usize]; m as usize]; // y, x
        grid[0][0] = 1;
        
        for row_idx in 0..(m as usize) {
            for col_idx in 0..(n as usize) {
                let above = if row_idx == 0 {
                    0
                } else {
                    grid[row_idx - 1][col_idx]
                };

                let prev = if col_idx == 0 {
                    0
                } else {
                    grid[row_idx][col_idx - 1]
                };

                if grid[row_idx][col_idx] == -1 {
                    grid[row_idx][col_idx] = above + prev;
                }
            }
        }

        grid[m as usize - 1][n as usize - 1]
    }
}

fn main() {
    assert_eq!(Solution::unique_paths(3, 7), 28);
    assert_eq!(Solution::unique_paths(3, 2), 3);
}