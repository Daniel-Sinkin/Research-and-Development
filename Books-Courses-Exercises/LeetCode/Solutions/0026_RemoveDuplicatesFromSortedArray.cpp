class Solution {
public:
    int removeDuplicates(vector<int>& nums) {
        if(nums.size() == 0 || nums.size() == 1)
            return nums.size();

        int currVal = nums[0];
        int leftPointer = 1;

        for(int i = 1; i < nums.size(); i++) {
            if(nums[i] != currVal) {
                nums[leftPointer] = nums[i];
                currVal = nums[i];
                leftPointer++;
            }
        }

        return leftPointer;
    }
};