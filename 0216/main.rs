struct Solution { }

use std::collections::VecDeque;

impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut q: VecDeque<(i32, Vec<i32>)> = VecDeque::new();
        let target_len: usize = (k as usize) - 1;
        
        for val in 1..10 {
            q.push_back((n - val, vec![val]));
        }
        
        while q.len() > 0 {
            let (value, combo) = q.pop_front().unwrap();
            let start_idx: i32 = *combo.last().unwrap() + 1;
            
            for idx in start_idx..10 {
                let tmp = value - idx;
                if tmp == 0 && combo.len() == target_len {
                    let mut res = combo.clone();
                    res.push(idx);
                    result.push(res);
                } else if tmp > 0 && combo.len() <= target_len {
                    let mut res = combo.clone();
                    res.push(idx);
                    q.push_back((tmp, res));
                }
            }
        }
        
        return result;
    }
}

fn main() {
    let res = Solution::combination_sum3(3, 7);
    assert!(res.contains(&vec![1, 2, 4]));
    assert!(res.len() == 1);

    let res = Solution::combination_sum3(3, 9);
    assert!(res.contains(&vec![1, 2, 6]));
    assert!(res.contains(&vec![1, 3, 5]));
    assert!(res.contains(&vec![2, 3, 4]));
    assert!(res.len() == 3);
}