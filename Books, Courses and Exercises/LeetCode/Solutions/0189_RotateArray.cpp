class Solution {
public:
    void rotate(vector<int>& nums, int k) {
        // if k == 0 or nums.size() <= 1 rotation doesn't do anything
        if(k != 0 && nums.size() > 1 && nums.size() != k) {
            int n = nums.size();
            // Assures that 0 <= k < n, note that rot^n(x, |x| * k) = x
            // for all k >= 0
            k = k % n;
            vector<int> helper(nums.begin(), nums.begin() + k);

            int s = 0;
            int temp;

            int counter = 0;

            bool isRunning = true;
            while(isRunning) {
                for(int i = 0; i < k; i++) {
                    if(counter < n) {
                        temp = helper[i];
                        helper[i] = nums[(i + (s + 1) * k) % n];
                        nums[(i + (s + 1) * k) % n] = temp;
                        counter++;
                    } else {
                        isRunning = false;
                        break;
                    }
                }
                s++;
            }
        }
    }
};