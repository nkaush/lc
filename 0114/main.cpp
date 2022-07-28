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
 
class Solution {
public:
    TreeNode* flatten_helper(TreeNode* root) {
        if (!root) {
            return nullptr;
        } else if (!root->left && !root->right) {
            return root;
        }
        
        TreeNode* left_right = flatten_helper(root->left);
        TreeNode* right_right = flatten_helper(root->right);
        
        if (root->left) {
            TreeNode* tmp = root->right;
            root->right = root->left;
            root->left = nullptr;
            left_right->right = tmp;
        }
        
        if (right_right) {
            return right_right;
        } else {
            return left_right;
        }
    }
    
    void flatten(TreeNode* root) {
        flatten_helper(root);
    }
};