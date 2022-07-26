int main() {
    
}

// Definition for a binary tree node.
struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode() : val(0), left(nullptr), right(nullptr) {}
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
    TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
};

#include <stack>

using std::stack;

class Solution {
public:
    int kthSmallest(TreeNode* root, int k) {
        stack<TreeNode*> s;
        
        TreeNode* curr = root;
        while (curr) {
            s.push(curr);
            curr = curr->left;
        }
        
        int index = 1;
        while (!s.empty()) {
            curr = s.top();
            s.pop();
            
            if (index++ == k) {
                return curr->val;
            }
            
            curr = curr->right;
            while (curr) {
                s.push(curr);
                curr = curr->left;
            }
        }
        
        return -1;
    }
};