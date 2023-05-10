class Solution {
public:
    int getRowStr(vector<int> row) {
        for(int i = 0; i < row.size(); i++) {
            if(row[i] == 0)
                return i;
        }
        return row.size();
    }

    // Gets the current minimum, always returning the first occurence on ties
    int getMinStable(vector<int> arr) {
        int min = INT_MAX;
        int retVal = 0;

        for(int i = 0; i < arr.size(); i++) {
            if(arr[i] < min) {
                min = arr[i];
                retVal = i;
            }
        }
        return retVal;
    }

    vector<int> kWeakestRows(vector<vector<int>>& mat, int k) {
        vector<int> retVal;

        vector<int> strength;

        for(int i = 0; i < mat.size(); i++) {
            strength.push_back(getRowStr(mat[i]));
        }

        int ind = 0;
        for(int i = 0; i < k; i++) {
            ind = getMinStable(strength);
            strength[ind] = mat[i].size() + 1;
            retVal.push_back(ind);
        }

        return retVal;
    }
};