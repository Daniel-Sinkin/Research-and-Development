class Solution {
public:
    int tribonacci(int n) {
        if(n == 0)
            return 0;
        if(n == 1)
            return 1;
        if(n == 2)
            return 1;

        int tribo[n + 1];
        tribo[0] = 0;
        tribo[1] = 1;
        tribo[2] = 1;

        for(int i = 3; i <= n; i++) {
            tribo[i] = tribo[i - 1] + tribo[i - 2] + tribo[i - 3];
        }
        
        return tribo[n];
    }
};