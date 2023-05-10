class Solution {
public:
    int lengthOfLongestSubstring(string s) {
        if(s == "")
            return 0;

        int retVal = 1;
        int curr = 0;
        uint bin = 0;
        vector<char> seenChars;
        for(int i = 0; i < s.size(); i++) {
            curr = 1;
            seenChars = {s[i]};
            for(int j = i + 1; j < s.size(); j++) {
                // Checks if s[j] has already been seen
                if(find(seenChars.begin(), seenChars.end(), s[j]) != seenChars.end()) 
                    j = s.size();
                else { // If not adds it to the vector and increases the count
                    seenChars.push_back(s[j]);
                    curr++;
                }
            }
            if(curr > retVal)
                retVal = curr;
        }
        return retVal;
    }
};