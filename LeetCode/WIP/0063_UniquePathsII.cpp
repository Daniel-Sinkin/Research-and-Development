class Solution {
public:
    map<pair<int, int>, int> myMap;
    int uniquePaths(vector<vector<int>>& obstacleGrid, int m, int n) {
        if(m == 1 || n == 1)
            return 1;

        cout << 2 << endl;

        map<pair<int, int>, int>::iterator iter = myMap.find(make_pair(m, n));
        if(iter != myMap.end()) {
            cout << 3 << endl;
            return iter->second;
        } else {
            cout << "4 with m = " << m << " and n = " << n << endl;
            int calc = 0;
            if(obstacleGrid[m - 1][n] != 1)
                calc += uniquePaths(obstacleGrid, m - 1, n);
            if(obstacleGrid[m][n - 1] != 1)
                calc += uniquePaths(obstacleGrid, m, n - 1);
                
            cout << "5 with m = " << m << " and n = " << n << endl;
            myMap[make_pair(m, n)] = calc;
            cout << "6" << endl;
            return calc;
        }
    }

    int uniquePathsWithObstacles(vector<vector<int>>& obstacleGrid) {
        int m = obstacleGrid.size();
        int n = obstacleGrid[0].size();

        cout << 1 << endl;

        return uniquePaths(obstacleGrid, m, n);        
    }
};