class Solution:
    def lengthOfLongestSubstring(self, s: str) -> int:
        first_ptr = 0
        second_ptr = 0
        
        longest = 0
        chars = dict()
        
        while second_ptr < len(s):
            curr = s[second_ptr]
            
            if curr in chars:
                longest = max(longest, len(chars))
                idx_curr = chars[curr]
                
                while first_ptr < idx_curr:
                    del chars[s[first_ptr]]
                    first_ptr += 1
                
                first_ptr += 1
            
            chars[curr] = second_ptr
            second_ptr += 1
                        
        return max(longest, len(chars))

if __name__ == "__main__":
    assert(Solution().lengthOfLongestSubstring("abcabcbb") == 3)
    assert(Solution().lengthOfLongestSubstring("bbbbb") == 1)
    assert(Solution().lengthOfLongestSubstring("pwwkew") == 3)
