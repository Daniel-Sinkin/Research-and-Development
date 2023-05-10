class Solution {
public:
    int jump(vector<int>& nums) {
        int n = nums.size() - 1;
        int temp;
        bool isUpdated = false;

        nums[n] = 0;
        for(int i = n - 1; i >= 0; i--) {
            if(nums[i] == 0) {
                nums[i] = -1;
            } else {
                temp = INT_MAX;
                isUpdated = false;
                for(int j = 1; j <= nums[i] && i + j <= n; j++) {
                    if(nums[i + j] >= 0) {
                        if(temp > nums[i + j]) {
                            temp = nums[i + j];
                            isUpdated = true;
                        }
                    }
                }
                if(!isUpdated)
                    nums[i] = -1;
                else
                    nums[i] = temp + 1;
            }
        }
        return nums[0];
    }
};