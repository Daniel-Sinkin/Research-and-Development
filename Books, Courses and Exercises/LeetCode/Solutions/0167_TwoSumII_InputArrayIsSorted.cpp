class Solution {
public:
    int binSearch(vector<int>& numbers, int left, int right, int target) {
        if(right == left) {
            return (numbers[left] == target) ? left : -1;
        }

        if(right == left + 1) {
            if(target == numbers[left]) {
                return left;
            }
            else if (target == numbers[right]) {
                return right;
            }
            else
                return -1;
        }
 
        int mid = (left + right) / 2;
        if(numbers[mid] == target) {
            return mid;
        } else if (numbers[mid] < target) {
            return binSearch(numbers, mid + 1, right, target);
        } else {
            return binSearch(numbers, left, mid - 1, target);
        }
    }

    vector<int> twoSum(vector<int>& numbers, int target) {
        // Fix i, x_i in numbers, we bin search for (target - x_i) in i+1, ..., n
        int bs;

        vector<int> retVal;

        for(int i = 0; i < numbers.size() - 1; i++) {
            bs = binSearch(numbers, i + 1, numbers.size() - 1, target - numbers[i]);
            if(bs != -1) {
                retVal.push_back(i + 1);
                retVal.push_back(bs + 1);
                return retVal;
            }
        }

        return retVal;
    }
};