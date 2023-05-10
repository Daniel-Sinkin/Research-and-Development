class Solution {
public:
    int pivotIndex(vector<int>& nums) {
        int RHS = 0;
        for(int i = 1; i < nums.size(); i++)
            RHS += nums[i];

        if(RHS == 0)
            return 0;

        int LHS = 0;
        for(int i = 1; i < nums.size(); i++) {
            LHS += nums[i  - 1];
            RHS -= nums[i];

            if(LHS == RHS)
                return i;
        }

        return -1;
    }
};