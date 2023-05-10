// The API isBadVersion is defined for you.
// bool isBadVersion(int version);

class Solution {
public:
    int firstBadVersion(int left, int right) {
        if(right < left || left == 0) {
            cout << "Catastrophic Error\n";
        }

        if(left == right) {
            if(isBadVersion(left))
                return left;
            else
                return INT_MAX;
        }

        if(right == left + 1) {
            if(isBadVersion(left))
                return left;
            else if(isBadVersion(right))
                return right;
            else
                return INT_MAX;
        }
 
        int mid = left / 2 + right / 2 + (left % 2) * (right % 2);
        int res;

        if(isBadVersion(mid)) {
            res = firstBadVersion(left, mid - 1);
            return (res < mid) ? res : mid;
        } else {
            res = firstBadVersion(mid + 1, right);
            return res;
        }
    }

    int firstBadVersion(int n) {
        return firstBadVersion(1, n);
    }
};