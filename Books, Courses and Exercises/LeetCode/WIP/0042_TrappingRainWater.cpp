class Solution {
public:
    int trap(vector<int>& height) {
        bool isEndCase = false;

        int n = height.size();

        int sum_curr = 0;
        int sum = 0;

        // finds the first wall of height >= h[i] *OR* reaches the end
        int i = 0;
        int j = 1;
        while(!isEnd) {
            j = i + 1;
            while(height[j] < height[i] && j < n) {
                sum_curr += height[i] - height[j];
                j++;
            }
            cout << "The new i is equal to " << j << endl;
            cout << "\tThe sum is currently " << sum << endl;
            if(j < n - 1) {
                sum += sum_curr;
                i = j;
            } else if (height[j] >= height[i]) {
                sum += sum_curr;  
            } else {
                isEndCase = true;
            }

            if(j < n - 1)
                i = j;

            if(j < n - 1) {
                i = j;
                sum += sum_curr;
            } else if (height[j] >= height[i]) {
                sum += sum_curr;
            } else {
                isEndCase = true;
            }
            sum_curr = 0;
        }

        if(isEndCase) {
            // We have a last index i, after which h[j] < h[i] including the very last wall
            // We can iterate through all and 
        }

        return sum;
    }
};
