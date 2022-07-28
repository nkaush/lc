from typing import Optional

# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution:
    def flatten_helper(self, root):
        if root is None:
            return None
        elif root.left is None and root.right is None:
            return root
        
        left_right = self.flatten_helper(root.left)
        right_right = self.flatten_helper(root.right)
                
        if root.left is not None:
            old_right = root.right
            root.right = root.left
            root.left = None
            left_right.right = old_right
            
        if right_right is not None:
            return right_right
        else:
            return left_right
        
    def flatten(self, root: Optional[TreeNode]) -> None:
        """
        Do not return anything, modify root in-place instead.
        """
        self.flatten_helper(root)

if __name__ == "__main__":
    pass