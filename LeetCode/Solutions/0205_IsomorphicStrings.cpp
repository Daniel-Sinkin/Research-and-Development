class Solution {
public:
    bool isIsomorphic(string s, string t) {
        if(s.size() != t.size())
            return false;

        bool d[s.size()];
        for(int i = 0; i < s.size(); i++)
            d[i] = false;

        for(int i = 0; i < s.size(); i++) {
            if(!d[i]){
                for(int j = i; j < s.size(); j++) {
                    //printf("(i, j) = (%d, %d)\n", i, j);
                    if (s[j] == s[i] || t[j] == t[i]) {
                        //printf("s[i] = %c, s[j] = %c, t[i] = %c, t[j] = %c\n", s[i], s[j], t[i], t[j]);
                        if(s[j] != s[i] || t[j] != t[i])
                            return false;
                        else
                            d[j] = true;
                    }
                }
            }
        }
        return true;
    }
};