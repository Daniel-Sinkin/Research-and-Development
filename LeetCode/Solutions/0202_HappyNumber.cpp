class Solution {
public:
    vector<int> seen;
    bool isHappy(int n) {
        if(n == 1)
            return true;
        
        int sum = 0;
        int temp = 0;
        while(n > 0) {
            temp = n % 10;
            sum += temp * temp;

            n /= 10;
        }
        if(find(seen.begin(), seen.end(), sum) != seen.end())
            return false;
        else
            seen.push_back(sum);
        
        return isHappy(sum);
    }
};