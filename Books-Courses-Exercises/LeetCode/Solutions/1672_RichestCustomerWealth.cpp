class Solution {
public:
    int maximumWealth(vector<vector<int>>& accounts) {
        vector<int> wealth;

        int helper;
        for(int i = 0; i < accounts.size(); i++) {
            helper = 0;
            for(int j = 0; j < accounts[i].size(); j++) {
                helper += accounts[i][j];
            }
            wealth.push_back(helper);
        }

        int retVal = INT_MIN;
        for(int i = 0; i < accounts.size(); i++) {
            if(wealth[i] > retVal)
                retVal = wealth[i];
        }
        return retVal;
    }
};