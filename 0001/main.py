from typing import List

class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        d = {}
        r = range(len(nums))
        
        for idx in r:
            d[target - nums[idx]] = idx
            
        for idx in r:
            search = d.get(nums[idx])
            if search != None and idx != search:
                return [search, idx]

if __name__ == "__main__":
    res = Solution().twoSum([2, 7, 11, 15], 9)
    assert(0 in res and 1 in res and len(res) == 2)

    res = Solution().twoSum([3, 2, 4], 6)
    assert(1 in res and 2 in res and len(res) == 2)

    res = Solution().twoSum([3, 3], 6)
    assert(0 in res and 1 in res and len(res) == 2)
