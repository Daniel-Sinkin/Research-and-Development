class Solution {
public:
    int numberOfSteps(int num) {
        int k = 0;
        int n = num;

        while(n > 0) {
            if(n % 2 == 0)
                n /= 2;
            else
                n--;
            k++;
        }
        return k;
    }
};