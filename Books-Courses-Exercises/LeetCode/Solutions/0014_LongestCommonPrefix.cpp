class Solution {
public:
    string longestCommonPrefix(vector<string>& strs) {
        string prefix = "";

        int position = 0;

        int n = strs.size();

        bool isChecking = true;
        
        for(int p = 0; p < strs[0].length(); p++) {
            char curr = strs[0][p];
            for(int i = 1; i < n; i++) {
                if(strs[i][p] != curr)
                    return prefix;
            }

            prefix += curr;
        }

        return prefix;
    }
};