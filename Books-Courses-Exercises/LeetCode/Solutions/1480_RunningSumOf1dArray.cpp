class Solution {
public:
    vector<int> runningSum(vector<int>& nums) {
        vector<int> retVal;
        int cummulative = 0;

        for(int i = 0; i < nums.size(); i++) {
            cummulative += nums[i];
            retVal.push_back(cummulative);
        }

        return retVal;
    }
};