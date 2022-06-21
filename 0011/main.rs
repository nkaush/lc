struct Solution { }

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max_volume: i32 = 0;
        let mut start: usize = 0;
        let mut end: usize = height.len() - 1;
        let mut tmp_v: i32;
        
        for diff in (0..height.len() as i32).rev() {
            if height[start] > height[end] {
                tmp_v = diff * height[end];
                if tmp_v > max_volume {
                    max_volume = tmp_v;
                }
                end -= 1;
            } else {
                tmp_v = diff * height[start];
                if tmp_v > max_volume {
                    max_volume = tmp_v;
                }
                start += 1;
            }
        }
        
        return max_volume;
    }
}

fn main() {
    assert_eq!(Solution::max_area(vec![1,8,6,2,5,4,8,3,7]), 49);
    assert_eq!(Solution::max_area(vec![1,1]), 1);
}