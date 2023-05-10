class Solution {
public:
    int modSearch(vector<int>& nums, int target, int index) {
        if(nums.size() == 0)
            return -1;

        if(nums.size() == 1) {
            if(nums[0] == target)
                return index;
            else
                return -1;
        }

        if(nums.size() == 2) {
            if(nums[0] == target)
                return index;
            else if (nums[1] == target)
                return index + 1;
            else
                return -1;
        }

        int mid = nums.size() / 2;
        printf("nums.size() = %d, mid = %d, index = %d\n", nums.size(), mid, index);

        if(target == nums[mid]) {
            return index + mid;
        } else if(target == nums[mid + 1]) {
            return index + mid + 1;
        } else if (target < nums[mid]) {
            vector<int> LHS(nums.begin(), nums.begin() + mid);
            return modSearch(LHS, target, index);
        } else {
            vector<int> RHS(nums.begin() + mid, nums.end());
            return modSearch(RHS, target, index + mid);
        }
    }

    int search(vector<int>& nums, int target) {
        return modSearch(nums, target, 0);
    }
};