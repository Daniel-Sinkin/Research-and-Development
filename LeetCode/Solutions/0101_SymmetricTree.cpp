/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode() : val(0), left(nullptr), right(nullptr) {}
 *     TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
 *     TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
 * };
 */
class Solution {
public:
    bool areSymmetric(TreeNode* left, TreeNode* right) {
        if(!left && !right)
            return true;

        if(!left && right || left && !right)
            return false;
        
        if(left->val == right->val) {
            return areSymmetric(left->right, right->left) && areSymmetric(left->left, right->right);
        }
        return false;
    }

    bool isSymmetric(TreeNode* root) {
        return areSymmetric(root->left, root->right);
    }
};