class Solution {
public:
    int strStr(string haystack, string needle) {
        for(int i = 0; i < haystack.length(); i++) {
            int j = 0;

            while(haystack[i + j] == needle[j]) {
                cout << "i " << i << " , j = " << j << endl;
                if(i + j == haystack.length())
                    return -1;
                    
                j++;
                if(j == needle.length())
                    return i;
            }
            if(j == needle.length())
                return i;
        }

        return -1;
    }
};