struct Solution { }

use std::collections::VecDeque;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.len() == 0 {
            return vec![];
        }
        
        let mut q: VecDeque<(&[char], String)> = VecDeque::new();
        let mut result: Vec<String> = Vec::new();

        let as_chars: Vec<char> = digits.chars().collect(); 
        q.push_back((&as_chars, String::new()));
        while q.len() > 0 {
            let (remaining_digits, letters) = q.pop_front().unwrap();

            let lookup: &str = match &remaining_digits[0] {
                '2' => "abc",
                '3' => "def",
                '4' => "ghi",
                '5' => "jkl",
                '6' => "mno",
                '7' => "pqrs",
                '8' => "tuv",
                '9' => "wxyz",
                _ => ""
            };

            for c in lookup.chars() {
                let mut cloned = letters.clone();
                cloned.push(c);

                if remaining_digits.len() == 1 {
                    result.push(cloned);
                } else {
                    q.push_back((&remaining_digits[1..], cloned));
                }
            }
        }

        return result;
    }
}

fn main() {
    let expected: Vec<String> = vec!["a".into(), "b".into(), "c".into()];
    assert_eq!(Solution::letter_combinations("2".into()), expected);

    let expected: Vec<String> = vec![
        "ad".into(), "ae".into(), "af".into(), "bd".into(), "be".into(), 
        "bf".into(), "cd".into(), "ce".into(), "cf".into()
    ];
    assert_eq!(Solution::letter_combinations("23".into()), expected);
}