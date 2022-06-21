struct Solution { }

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::new();
        
        for idx in 0..nums.len() {
            map.insert(target - nums[idx], idx);
        }
        
        for curr_idx in 0..nums.len() {
            if let Some(prev_idx) = map.get(&nums[curr_idx]) {
                if prev_idx != &curr_idx {
                    return vec![*prev_idx as i32, curr_idx as i32];
                }
            }
        }
        
        return vec![]; // we should never get here!
    }
}

fn main() {
    let res = Solution::two_sum(vec![2, 7, 11, 15], 9);
    assert!(res.contains(&0));
    assert!(res.contains(&1));
    assert!(res.len() == 2);

    let res = Solution::two_sum(vec![3, 2, 4], 6);
    assert!(res.contains(&1));
    assert!(res.contains(&2));
    assert!(res.len() == 2);

    let res = Solution::two_sum(vec![3, 3], 6);
    assert!(res.contains(&0));
    assert!(res.contains(&1));
    assert!(res.len() == 2);
}