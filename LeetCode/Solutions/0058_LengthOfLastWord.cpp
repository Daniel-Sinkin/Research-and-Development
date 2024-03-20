class Solution {
public:
    int lengthOfLastWord(string s) {
        int curr = 0;
        bool newWord = true;
        for(int i = 0; i < s.size(); i++) {
            if(s[i] != ' ') { // On letter
                if(newWord) {
                    curr = 1;
                    newWord = false;
                } else {
                    curr++;
                }
            }
            else { // On ' '
                newWord = true;
            }
        }

        return curr;
    }
};