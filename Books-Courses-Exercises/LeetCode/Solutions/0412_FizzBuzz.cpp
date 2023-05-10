class Solution {
public:
    vector<string> fizzBuzz(int n) {
        vector<string> retVal;
        // Transvering backwards because it's faster to append elements to the end of the vector
        // Output is identical if I do the i = 0, ..., n direction and reverse array at the end
        for(int i = 1; i <= n; i++) {
            if(i % 3 == 0 && i % 5 == 0)
                retVal.push_back("FizzBuzz");
            else if(i % 3 == 0)
                retVal.push_back("Fizz");
            else if(i % 5 == 0)
                retVal.push_back("Buzz");
            else
                retVal.push_back(to_string(i));
        }
        return retVal;
    }
};