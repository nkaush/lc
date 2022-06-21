from typing import List

class Solution:
    def maxArea(self, height: List[int]) -> int:
        max_volume = 0
        start = 0
        end = len(height) - 1
        
        for diff in range(end, 0, -1):
            if height[start] > height[end]:
                tmp_v = height[end] * diff
                if tmp_v > max_volume:
                    max_volume = tmp_v
                end -= 1
            else:
                tmp_v = height[start] * diff
                if tmp_v > max_volume:
                    max_volume = tmp_v
                start += 1
                
        return max_volume

if __name__ == "__main__":
    assert(Solution().maxArea([1,8,6,2,5,4,8,3,7]) == 49)
    assert(Solution().maxArea([1,1]) == 1)