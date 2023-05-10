class Solution {
public:
    int titleToNumber(string columnTitle) {
        // This basically has us calculate in base 26
        // with digits being A..Z
        int pow = 1;
        int retVal = 0;
        for(int i = columnTitle.size() - 1; i >= 0; i--) {
            retVal += (char(columnTitle[i]) - 64) * pow;
            if(i > 0)
                pow *= 26;
        }

        return retVal;
    }
};