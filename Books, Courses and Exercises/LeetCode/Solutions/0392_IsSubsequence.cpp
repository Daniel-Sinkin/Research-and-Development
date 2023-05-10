class Solution {
public:
    bool isSubsequence(string s, string t) {
        int p1 = 0;
        int p2 = 0;

        while(p1 < s.size()) {
            while(s[p1] != t[p2]) {
                if(p2 >= t.size())
                    return false;

                p2++;
            }

            p1++;
            p2++;
        }

        return true;
    }
};