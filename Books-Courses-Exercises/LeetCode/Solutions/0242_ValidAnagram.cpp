class Solution {
public:
    bool isAnagram(string s, string t) {
        map<char, int> myMap;

        if(s.size() != t.size())
            return false;

        if(s == "")
            return true;

        for(int i = 0; i < s.size(); i++) {
            myMap[s[i]]++;
            myMap[t[i]]--;
        }

        for(char c = 'a'; c < 'z'; c++) {
            if(myMap[c] != 0)
                return false;
        }

        return true;
    }
};