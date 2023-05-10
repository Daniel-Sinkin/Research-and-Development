class Solution {
public:
    bool canJump(vector<int>& nums) {
        int n = nums.size();
        if(n <= 1)
            return true;
        if(n == 2)
            return (nums[0] > 0);

        bool canJump[n];
        canJump[n - 1] = true;
        canJump[n - 2] = (nums[n - 2] > 0);

        bool helper = false;
        for(int i = nums.size() - 3; i >= 0; i--) {
            if(nums[i] == 0)
                canJump[i] = false;
            else {
                helper = false;
                for(int j = 1; j <= nums[i] && i + j < nums.size(); j++) {
                    if(canJump[i + j]) {
                        helper = true;
                        j = nums[i];
                    }
                }
                canJump[i] = helper;
            }
        }

        return canJump[0];
    }
};