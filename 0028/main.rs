struct Solution { }

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.len() == 0 {
            return 0;
        } else if needle.len() > haystack.len() {
            return -1;
        }
        
        let h_chars: Vec<char> = haystack.chars().collect();
        let start: char = needle.chars().nth(0).unwrap();
        let mut res;

        for idx in 0..(haystack.len() - needle.len() + 1) {
            if h_chars[idx] == start {
                res = true;
                
                for (i, c) in needle.chars().enumerate() {
                    res &= h_chars[idx + i] == c;
                }
                
                if res {
                    return idx as i32;
                }
            }
        }
        
        return -1;
    }
}

fn main() {
    assert_eq!(Solution::str_str("hello".into(), "ll".into()), 2);
    assert_eq!(Solution::str_str("aaaaa".into(), "bba".into()), -1);
}