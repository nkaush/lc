struct Solution { }

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let m = obstacle_grid.len();
        let n = obstacle_grid[0].len();
        let mut grid: Vec<Vec<i32>> = vec![vec![-1; n]; m];
        grid[0][0] = 1;
        
        for row_idx in 0..m {
            for col_idx in 0..n {
                if obstacle_grid[row_idx][col_idx] == 1 {
                    grid[row_idx][col_idx] = 0;
                    continue;
                }
                
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
    let case1 = vec![vec![0,0,0],vec![0,1,0],vec![0,0,0]];
    assert_eq!(Solution::unique_paths_with_obstacles(case1), 2);

    let case2 = vec![vec![0,1],vec![0,0]];
    assert_eq!(Solution::unique_paths_with_obstacles(case2), 1);
}