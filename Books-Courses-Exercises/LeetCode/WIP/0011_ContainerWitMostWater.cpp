class Solution {
public:
    int maxArea(vector<int>& height) {
        int waterMax = 0;

        int leftMax = 0;
        int rightMax = height.size() - 1;

        int left = leftMax;
        int right = rightMax;

        // The currently selected container has height min{left, right} * (right - left)
        int h = (height[left] < height[right]) ? height[right] : height[left];
        int water = h * (right - left);


        while(height[left] < height[right]) {
            left++;
            h = (height[left] < height[right]) ? height[right] : height[left];
            water = h * (right - left);

            if (water > waterMax) {
                leftMax = left;
                rightMax = right;
                waterMax = water;
            }
        }

        while(height[right] < height[left]) {
            right--;
            h = (height[left] < height[right]) ? height[right] : height[left];
            water = h * (right - left);

            if (water > waterMax) {
                leftMax = left;
                rightMax = right;
                waterMax = water;
            }
        }
    }
};