class Solution {
public:
    int longestPalindrome(string s) {
        int counter[52];

        for(int i = 0; i < 52; i++)
            counter[i] = 0;

        bool existsOdd = false;
        int evenCounter = 0;

        for(int i = 0; i < s.size(); i++) {
            if(97 <= s[i]) // Lowercase Letter
                counter[s[i] - 97]++;
            else // Uppercase Letter
                counter[s[i] - 65 + 26]++;
        }

        for(int i = 0; i < 52; i++) {
            if(counter[i] % 2 == 1)
                existsOdd = true;
            
            evenCounter += counter[i] - (counter[i] % 2);
        }

        return (existsOdd) ? evenCounter + 1 : evenCounter;
    }
};