class Solution {
public:
    int reverse(int x) {
        bool isNegative = (x < 0);
        if(isNegative) {
            if(x == -2147483648)
                return 0;

            x = x * (-1);
        }

        int MAX_INT = 2147483647;
        int retVal = 0;

        while(x != 0) {
            int d = (x % 10);

            // The following code checks for overflow, on undefined behaviour we return 0
            if (retVal > MAX_INT / 10 || (retVal == MAX_INT / 10 && d > 7)) {
                return 0;
            }

            retVal = retVal * 10 + d;

            x /= 10;
        }

        if(isNegative)
            return -1 * retVal;
        
        return retVal;
    }
};