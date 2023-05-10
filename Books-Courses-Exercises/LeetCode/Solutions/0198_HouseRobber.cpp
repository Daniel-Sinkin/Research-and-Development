class Solution {
public:
    map<int, int> vals;
    int robHelper(vector<int>& nums, int position) {
        if(nums.size() == position) {
            return 0;
        } else if(nums.size() - position == 1) {
            return nums[position];
        } else if (nums.size() - position == 2) {
            return max(nums[position], nums[position + 1]);
        } else {
            map<int, int>::iterator iter = vals.find(position);
            if(iter != vals.end()) {
                return iter->second;
            } else {
                int temp1 = nums[position] + robHelper(nums, position + 2);
                int temp2 = robHelper(nums, position + 1);

                int maxVal = max(temp1, temp2);
                vals[position] = maxVal;
                return maxVal;
            }
        }
    }

    int rob(vector<int>& nums) {
        // rob({x_1, ..., x_k}) = mfx{nums[x_1] + rob{x_3, ..., x_k}, rob{x_2, ..., x_k}}
        // So for your element you have one branch; nums.length <= 100 means at most 100 stack size
        return robHelper(nums, 0);
    }
};