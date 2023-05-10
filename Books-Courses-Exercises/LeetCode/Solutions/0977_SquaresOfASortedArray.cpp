class Solution {
public:
    vector<int> sortedSquares(vector<int>& nums) {
        // This is basically merging two sorted lists
        // We have
        // [A, B], where A < 0, B >= 0 and A, B sorted asc.
        // Then after Squareing
        // [A^2, B^2] where A^2 > 0, B^2 >= 0 and A sort desc., B sorted asc.
        // So we have pointers at their boundary and then merge in <- 
        // and -> direction
        int ptr = -1;
        vector<int> retVal;

        int curr;

        bool isNegative = false;
        for(int i = 0; i < nums.size(); i++) {
            if(nums[i] >= 0) {
                ptr = i - 1;
                isNegative = false;
                i = nums.size();
            } else {
                nums[i] *= nums[i];
                ptr = nums.size() - 1;
                isNegative = true;
            }
        }

        // All Positive
        if(ptr == -1) {
            for(int i = 0; i < nums.size(); i++) {
                nums[i] *= nums[i];
            }
            return nums;
        } // All Negative
        else if(ptr == nums.size() - 1) {
            for(int i = 0; i < nums.size(); i++) {
                reverse(nums.begin(), nums.end());
                return nums;
            }
        }

        int x = ptr;
        int y = ptr + 1;
        curr = -1;
        while(true) {
            if(x < 0) {
                if(curr == -1)
                    curr = nums[y] * nums[y];
                //cout << "x < 0\n";
                //printf("(x, y, nums[x], nums[y], ptr) = (-1,%d,undefined,%d,%d)\n", y, nums[y], ptr);
                retVal.push_back(curr);
                for(int i = y+1; i < nums.size(); i++) {
                    retVal.push_back(nums[i] * nums[i]);
                }
                return retVal;
            } else if (y == nums.size()) {
                //cout << "y = nums.size()\n";
                while(x >= 0) {
                    //printf("(x, y, nums[x], nums[y], ptr) = (%d,nums.size(),%d,undefined,%d)\n", x, nums[x], ptr);
                    retVal.push_back(nums[x]);
                    x--;
                }
                return retVal;
            } else {
                if(curr == -1)
                    curr = nums[y] * nums[y];

                //cout << "else\n";
                //printf("(x, y, nums[x], nums[y], ptr) = (%d,%d,%d,%d,%d)\n", x, y, nums[x], nums[y], ptr);
                if(curr < nums[x]) {
                    retVal.push_back(curr);
                    curr = -1;
                    y++;
                } else {
                    retVal.push_back(nums[x]);
                    x--;
                }
            }
        }

        return retVal;
    }
};