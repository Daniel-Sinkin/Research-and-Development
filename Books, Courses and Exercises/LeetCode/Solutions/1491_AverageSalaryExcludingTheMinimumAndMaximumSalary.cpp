class Solution {
public:
    double average(vector<int>& salary) {
        int min = INT_MAX;
        int max = INT_MIN;

        int sum = 0;

        if(salary.size() <= 2)
            return 0;

        for(int i = 0; i < salary.size(); i++) {
            if(salary[i] < min)
                min = salary[i];
            if(salary[i] > max)
                max = salary[i];

            sum += salary[i];
        }

        double corrSum = (double)(sum - max - min);

        // printf("sum = %d, max = %d, min = %d, corrSum = %f, salary.size() = %d", sum, max, min, corrSum, salary.size());

        return corrSum / (salary.size() - 2);
    }
};