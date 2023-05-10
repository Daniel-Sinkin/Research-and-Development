class Solution {
public:
    void moveZeroes(vector<int>& nums) {
        int ptr = 0;

        while(nums[ptr] != 0) {
            ptr++;
            if(ptr == nums.size())
                break;
        }

        for(int i = 0; i < nums.size() && ptr < nums.size(); i++) {
            if(nums[i] != 0 && nums[ptr] == 0 && ptr < i) {
                nums[ptr] = nums[i];
                nums[i] = 0;
                ptr++;
            }
        }
    }
};