from typing import Optional
from collections import deque

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution:
    def isValidBST(self, root: Optional[TreeNode]) -> bool:
        nodes = deque()
        
        nodes.append(root)
        
        tmp = root.left
        while tmp != None:
            nodes.append(tmp)
            tmp = tmp.left  
        
        previous_val = None
        while len(nodes) > 0:
            curr = nodes.pop()
            if previous_val is not None and previous_val >= curr.val:
                return False
            
            previous_val = curr.val
            
            # add right + all left children
            if curr.right is not None:                
                tmp = curr.right
                
                while tmp is not None:
                    nodes.append(tmp)
                    tmp = tmp.left
        
        return True

if __name__ == "__main__":
    pass