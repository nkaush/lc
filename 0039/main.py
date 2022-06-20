from collections import deque
from typing import List

class Solution:
    def combinationSum(self, candidates: List[int], target: int) -> List[List[int]]:
        result = list()
        q = deque()
        
        for c in candidates:
            if c == target:
                result.append([c])
            else:
                q.append((target - c, [c]))
                
        while len(q) > 0:
            val, combo = q.pop()
            
            for c in candidates:
                if c < combo[-1]: 
                    continue
                
                tmp = val - c
                if tmp == 0:
                    result.append(sorted(combo[:] + [c]))
                elif tmp > 0:
                    q.append((tmp, combo[:] + [c]))
            
        return result

if __name__ == "__main__":
    res = Solution().combinationSum([2, 3, 6, 7], 7)
    assert([7] in res and [2, 2, 3] in res)
