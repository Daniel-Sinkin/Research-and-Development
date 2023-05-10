class Solution {
public:
    int searchInsert(vector<int>& nums, int target) {
        int left = 0;
        int right = nums.size() - 1;
        int mid;
        while(true) {
            mid = (left + right) / 2;
            printf("(left, mid, right) = (%d,%d,%d)\n", left, mid, right);
            if(left == right) {
                if(nums[left] == target)
                    return left;
                else if (nums[left] < target)
                    return left + 1;
                else
                    return left;
            }
            
            if(right == left + 1) {
                if(target < nums[left])
                    return left;
                else if(target == nums[left])
                    return left;
                else if(target == nums[right])
                    return right;
                else if (target > nums[right])
                    return right + 1;
                else
                    cout << "Catastrophic Error!\n";
            }

            if(nums[mid] == target)
                return mid;
            else if (nums[mid] < target)
                left = mid + 1;
            else
                right = mid - 1;
        }
        return -1;
    }
};