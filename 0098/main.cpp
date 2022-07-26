int main() {

}

struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode() : val(0), left(nullptr), right(nullptr) {}
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
    TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
};

#include <stack>

class Solution {
public:
    bool isValidBST(TreeNode* root) {
        std::stack<TreeNode*> s;
        
        TreeNode* curr = root;
        while (curr != NULL) {
            s.push(curr);
            curr = curr->left;
        }
        
        int* prev = NULL;
        while (!s.empty()) {
            curr = s.top();
            s.pop();
            
            if (prev && *prev >= curr->val) {
                return false;
            }
            
            prev = &(curr->val);            
            curr = curr->right;
            while (curr) {
                s.push(curr);
                curr = curr->left;
            }
        }
        
        return true;
    }
};