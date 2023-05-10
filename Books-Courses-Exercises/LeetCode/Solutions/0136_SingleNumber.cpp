class Solution {
public:
    int singleNumber(vector<int>& nums) {
        // Inspired by https://www.youtube.com/watch?v=c33AZBnRHks
        // in particular this solution https://github.com/oisyn/parkerwords
        int XOR = 0;
        for(int n : nums) {
            XOR ^= n;
        }
        return XOR;
    }
};