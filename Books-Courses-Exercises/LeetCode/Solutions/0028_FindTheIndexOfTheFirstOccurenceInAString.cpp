class Solution {
public:
    int strStr(string haystack, string needle) {
        int checkNext;
        int j = 0;
        int i_temp, j_temp;
        bool hasChecked = false;
        bool isRunning;
        for(int i = 0; i < haystack.size(); i++) {
            cout << i << endl;
            if(haystack[i] == needle[j]) {
                i_temp = 1;
                j_temp = 1;
                isRunning = i + i_temp < haystack.size() && j + j_temp < needle.size();
                while(isRunning && haystack[i + i_temp] == needle[j + j_temp]) {
                    if(!hasChecked) {
                        if(haystack[i + i_temp] == needle[j]) {
                            hasChecked = true;
                            checkNext = i + i_temp - 1;
                        }
                    }
                    i_temp++;
                    j_temp++;
                    isRunning = i + i_temp < haystack.size() && j + j_temp < needle.size();
                }

                if(j + j_temp == needle.size()) {
                    return i;
                } else if (hasChecked) {
                    i = checkNext;
                }
                hasChecked = false;
            }
            j = 0;
        }

        return -1;
    }
};