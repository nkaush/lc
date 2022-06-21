struct Solution { }

use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut start_iter = s.chars();
        let mut start: usize = 0;
        
        let mut longest: usize = 0;
        let mut map: HashMap<char, usize> = Default::default();
        
        for (idx, curr) in s.chars().enumerate() {
            if map.contains_key(&curr) {
                if map.len() > longest {
                    longest = map.len();
                }
                
                let idx_curr = *map.get(&curr).unwrap();
                while start < idx_curr {
                    map.remove(&start_iter.next().unwrap());
                    start += 1;
                }
                
                start_iter.next();
                start += 1;
            }
            
            map.insert(curr, idx);
        }
        
        if map.len() > longest {
            longest = map.len();
        }
        
        return longest as i32;
    }
}

fn main() {
    assert_eq!(Solution::length_of_longest_substring("abcabcbb".into()), 3);
    assert_eq!(Solution::length_of_longest_substring("bbbbb".into()), 1);
    assert_eq!(Solution::length_of_longest_substring("pwwkew".into()), 3);
}