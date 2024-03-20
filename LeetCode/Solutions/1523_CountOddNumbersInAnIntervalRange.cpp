class Solution {
public:
    int countOdds(int low, int high) {
        return (low % 2) * (high % 2) + (high - low + 1) / 2;
    }
};