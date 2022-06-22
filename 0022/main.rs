struct Solution { }

use std::collections::VecDeque;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut q: VecDeque<(String, i32, i32)> = VecDeque::new();
        let mut result: Vec<String> = Vec::new();
        let target_len: usize = (n as usize) * 2;
        
        q.push_back(("(".into(), 1, 0));
        
        while q.len() > 0 {
            let (curr, open, closed) = q.pop_front().unwrap();
            
            if curr.len() == target_len {
                result.push(curr);
                continue;
            }
            
            if open < n {
                let mut cloned = curr.clone();
                cloned.push('(');
                q.push_back((cloned, open + 1, closed));
            }
            
            if closed < open {
                let mut cloned = curr.clone();
                cloned.push(')');
                q.push_back((cloned, open, closed + 1));
            }
        }
        
        return result;
    }
}
