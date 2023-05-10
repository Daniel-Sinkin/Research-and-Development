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
    bool hasPathSum(TreeNode* root, int targetSum) {
        if(!root)
            return false;

        bool retVal = false;

        if(root->left)
            retVal = retVal || hasPathSum(root->left, targetSum - root->val);

        if(root->right)
            retVal = retVal || hasPathSum(root->right, targetSum - root->val);

        if(!root->left && !root->right)
            retVal = (root->val == targetSum);

        return retVal;
    }
};