class Solution {
public:
    map<pair<int, int>, int> myMap;
    int uniquePaths(int m, int n) {
        if(m <= 1 || n <= 1)
            return 1;

        if(m > n)
            return uniquePaths(n, m);

        if(m == n)
            return 2 * uniquePaths(m - 1, n);

        map<pair<int, int>, int>::iterator iter = myMap.find(make_pair(m, n));
        if(iter != myMap.end()) {
            return iter->second;
        } else {
            int calc = uniquePaths(m - 1, n) + uniquePaths(m, n - 1);
            myMap[make_pair(m, n)] = calc;
            return calc;
        }
    }
};