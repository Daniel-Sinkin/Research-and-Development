class Solution {
public:
    vector<int> rowAndMaximumOnes(vector<vector<int>>& mat) {
        int max_count = 0;
        int max_index = 0;
        for(int m = 0; m < mat.size(); m++) {
            cout << m << endl;
            int curr = 0;
            for(int n = 0; n < mat[m].size(); n++) {
                if(mat[m][n] == 1)
                    curr++;
            }
            
            if(curr > max_count) {
                max_index = m;
                max_count = curr;
            }
        }

        vector<int> vec {max_index, max_count};
        
        return vec;
    }
};