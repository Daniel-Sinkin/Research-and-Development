class Solution {
public:
    string intToRoman(int num) {
        string retVal = "";
        while(num > 0) {
            if(num >= 1000) {
                num -= 1000;
                retVal = retVal + "M";
            } else if (num >= 900) {
                num -= 900;
                retVal = retVal + "CM";
            } else if (num >= 500) {
                num -= 500;
                retVal = retVal + "D";
            } else if (num >= 400) {
                num -= 400;
                retVal = retVal + "CD";
            } else if (num >= 100) {
                num -= 100;
                retVal = retVal + "C";
            } else if (num >= 90) {
                num -= 90;
                retVal = retVal + "XC";
            } else if (num >= 50) {
                num -= 50;
                retVal = retVal + "L";
            } else if (num >= 40) {
                num -= 40;
                retVal = retVal + "XL";
            } else if (num >= 10) {
                num -= 10;
                retVal = retVal + "X";
            } else if (num >= 9) {
                num -= 9;
                retVal = retVal + "IX";
            } else if (num >= 5) {
                num -= 5;
                retVal = retVal + "V";
            } else if (num >= 4) {
                num -= 4;
                retVal = retVal + "IV";
            } else if (num >= 1) {
                num -= 1;
                retVal = retVal + "I";
            }
        }
        return retVal;
    }
};