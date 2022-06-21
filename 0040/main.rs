struct Solution { }

use std::collections::{VecDeque, HashSet};

impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut q: VecDeque<(i32, Vec<i32>, usize)> = VecDeque::new();
        let mut result: HashSet<Vec<i32>> = HashSet::new();
        let mut prior: HashSet<Vec<i32>> = HashSet::new();
        let mut candidates = candidates.clone();
        candidates.sort();
        
        for (idx, c) in candidates.iter().enumerate() {
            if c == &target {
                result.insert(vec![*c]);
            } else {
                q.push_back((target - c, vec![*c], idx));
            }
        }
        
        while q.len() > 0 {
            let (value, combo, start_idx) = q.pop_front().unwrap();
            
            for idx in (start_idx + 1)..candidates.len() {                
                let tmp = value - candidates[idx];
                if tmp == 0 {
                    let mut cloned = combo.clone();
                    cloned.push(candidates[idx]);
                    result.insert(cloned);
                } else if tmp > 0 {
                    let mut cloned = combo.clone();
                    cloned.push(candidates[idx]);
                    
                    if !prior.contains(&cloned) {
                        q.push_back((tmp, cloned.clone(), idx));
                        prior.insert(cloned);
                    }
                }
            }
        }
        
        return result.into_iter().collect();
    }
}

fn main() {
    let res = Solution::combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8);
    assert!(res.contains(&vec![1, 1, 6]));
    assert!(res.contains(&vec![1, 2, 5]));
    assert!(res.contains(&vec![1, 7]));
    assert!(res.contains(&vec![2, 6]));

    let res = Solution::combination_sum2(vec![2, 5, 2, 1, 2], 5);
    assert!(res.contains(&vec![1, 2, 2]));
    assert!(res.contains(&vec![5]));
}