struct Solution { }

use std::collections::VecDeque;

impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let mut visits = vec![1; is_connected.len()];
        let mut num_provinces = 0;
        
        for province in 0..is_connected.len() {
            if visits[province] == 1 {
                num_provinces += 1;
                Solution::dfs(province, &is_connected, &mut visits);
            }
        }
        
        num_provinces
    }
    
    fn bfs(start: usize, matrix: &Vec<Vec<i32>>, visits: &mut Vec<i32>) {
        let mut q = VecDeque::new();
        q.push_back(start);
        
        while q.len() > 0 {
            let curr = q.pop_front().unwrap();
            visits[curr] = 0;
            
            for province in 0..matrix.len() {
                if visits[province] == 1 && matrix[curr][province] == 1 {
                    q.push_back(province);
                }
            }
        }
    }
    
    fn dfs(start: usize, matrix: &Vec<Vec<i32>>, visits: &mut Vec<i32>) {
        visits[start] = 0;

        for province in 0..matrix.len() {
            if visits[province] == 1 && matrix[start][province] == 1 {
                Solution::dfs(province, matrix, visits);
            }
        }
    }
}

fn main() {
    
}