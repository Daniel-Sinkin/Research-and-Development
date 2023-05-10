class Solution {
public:
    int climbStairs(int n) {
        if(n <= 1)
            return 1;
        if(n == 2)
            return 2;

        vector<int> helper;
        helper.push_back(1);
        helper.push_back(2);
        for(int i = 2; i < n; i++) {
            helper.push_back(helper[i - 1] + helper[i - 2]);
        }

        return helper[n - 1];
    }
};