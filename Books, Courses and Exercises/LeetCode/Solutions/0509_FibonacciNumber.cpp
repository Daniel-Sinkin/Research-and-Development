class Solution {
public:
    int fib(int n) {
        if(n == 0)
            return 0;
        else if (n == 1)
            return 1;

        int fibs[n + 1];
        fibs[0] = 0;

        fibs[1] = 1;

        for(int i = 2; i <= n; i++) {
            fibs[i] = fibs[i - 1] + fibs[i - 2];
        }

        return fibs[n];
    }
};