class Solution {
public:
    bool canConstruct(string ransomNote, string magazine) {
        int count[26];
        
        for(int i = 0; i < 26; i++) {
            count[i] = 0;
        }

        // Counts how often letter appears in ransomNote
        for(int i = 0; i < ransomNote.length(); i++) {
            count[ransomNote[i] - 97]++;
        }
        
        // Removes each occurence in magazine count of that letter
        for(int i = 0; i < magazine.length(); i++) {
            count[magazine[i] - 97]--;
        }

        for(int i = 0; i < 26; i++) {
            if(count[i] > 0)
                return false;
        }

        return true;
    }
};