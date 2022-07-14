struct Solution { }

use std::cmp::min;

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let mut scores = grid.clone();

        for row_idx in 0..grid.len() {
            for col_idx in 0..grid[row_idx].len() {
                let to_add = if row_idx == 0 && col_idx == 0 {
                    0
                } else if row_idx == 0 {
                    scores[row_idx][col_idx - 1]
                } else if col_idx == 0 {
                    scores[row_idx - 1][col_idx]
                } else {
                    min(scores[row_idx-1][col_idx], scores[row_idx][col_idx-1])
                };

                scores[row_idx][col_idx] += to_add;
            }
        }

        scores[scores.len() - 1][scores[0].len() - 1]
    }
}

fn main() {
    let case1 = vec![vec![1,3,1],vec![1,5,1],vec![4,2,1]];
    assert_eq!(Solution::min_path_sum(case1), 7);

    let case2 = vec![vec![1,2,3],vec![4,5,6]];
    assert_eq!(Solution::min_path_sum(case2), 12);

    let case3 = vec![
        vec![0,2,2,6,4,1,6,2,9,9,5,8,4,4],
        vec![0,3,6,4,5,5,9,7,8,3,9,9,5,4],
        vec![6,9,0,7,2,2,5,6,3,1,0,4,2,5],
        vec![3,8,2,3,2,8,8,7,5,9,6,3,4,5],
        vec![4,0,1,3,9,2,0,1,6,7,9,2,8,9],
        vec![6,2,7,9,0,9,5,2,7,5,1,4,4,7],
        vec![9,8,3,3,0,6,8,0,8,8,3,5,7,3],
        vec![7,7,4,5,9,1,5,0,2,2,2,1,7,4],
        vec![5,1,3,4,1,6,0,4,3,8,4,3,9,9],
        vec![0,6,4,9,4,1,5,5,4,2,5,7,4,0],
        vec![0,1,6,6,4,9,2,7,8,2,1,3,3,7],
        vec![8,4,9,9,2,3,8,6,6,5,4,1,7,9]
    ];
    assert_eq!(Solution::min_path_sum(case3), 63);
}