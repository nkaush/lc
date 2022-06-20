struct Solution { }

use std::collections::VecDeque;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut q: VecDeque<(i32, Vec<i32>)> = VecDeque::new();
        
        for c in candidates.iter() {
            if c == &target {
                result.push(vec![*c]);
            } else {
                q.push_back((target - c, vec![*c]));
            }
        }
        
        while q.len() > 0 {
            let (value, combo) = q.pop_front().unwrap();
            
            let last: &i32 = combo.last().unwrap();
            for c in candidates.iter() {
                if c < last {
                    continue;
                }
                
                let tmp = value - c;
                if tmp == 0 {
                    let mut res = combo.clone();
                    res.push(*c);
                    result.push(res);
                } else if tmp > 0 {
                    let mut res = combo.clone();
                    res.push(*c);
                    q.push_back((tmp, res));
                }
            }
        }
        
        return result;
    }
}

fn main() {
    let result = Solution::combination_sum(vec![2, 3, 6, 7], 7);
    assert!(result.contains(&vec![7i32]));
    assert!(result.contains(&vec![2i32, 2, 3]));
}