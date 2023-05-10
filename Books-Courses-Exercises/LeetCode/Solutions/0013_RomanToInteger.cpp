class Solution {
public:
    int romanToInt(string s) {
        int retVal = 0;
        for(int i = 0; i < s.length(); i++) {
                switch(s[i]) {
                case 'M':
                    retVal += 1000;
                    break;
                case 'D':
                    retVal += 500;
                    break;
                case 'C':
                    if (i + 1 < s.length()) {
                        if (s[i + 1] == 'M') {
                            retVal += 900;
                            i++;
                        }
                        else if (s[i + 1] == 'D') {
                            retVal += 400;
                            i++;
                        }
                        else
                            retVal += 100;
                    } else {
                        retVal += 100;
                    }
                    break;
                case 'L':
                    retVal += 50;
                    break;
                case 'X':
                    if (i + 1 < s.length()) {
                        if (s[i + 1] == 'C') {
                            retVal += 90;
                            i++;
                        }
                        else if (s[i + 1] == 'L') {
                            retVal += 40;
                            i++;
                        }
                        else
                            retVal += 10;
                    } else {
                        retVal += 10;
                    }
                    break;
                case 'V':
                    retVal += 5;
                    break;
                case 'I':
                    if (i + 1 < s.length()) {
                        if (s[i + 1] == 'X') {
                            retVal += 9;
                            i++;
                        }
                        else if (s[i + 1] == 'V') {
                            retVal += 4;
                            i++;
                        }
                        else
                            retVal += 1;
                    } else {
                        retVal += 1;
                    }
                    break;
            }

        }

        return retVal;
    }
};