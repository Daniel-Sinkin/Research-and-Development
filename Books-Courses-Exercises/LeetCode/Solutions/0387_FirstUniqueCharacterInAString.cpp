class Solution {
public:
    int firstUniqChar(string s) {
        int index = INT_MAX;
        map<char, int> myMap;
        for(int i = 0; i < s.size(); i++) {
            if(myMap.count(s[i]) > 0) {
                myMap[s[i]] = -1;
            }
            else {
                myMap[s[i]] = i;
            }
        }

        int temp;
        map<char, int>::iterator it = myMap.begin();
        while(it != myMap.end()) {
            temp = it->second;
            if(temp >= 0) {
                if(index > temp)
                    index = temp;
            }

            it++;
        }

        if(index == INT_MAX)
            return -1;
        else
            return index;
    }
};