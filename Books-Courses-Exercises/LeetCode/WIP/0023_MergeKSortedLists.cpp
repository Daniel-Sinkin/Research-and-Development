/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode() : val(0), next(nullptr) {}
 *     ListNode(int x) : val(x), next(nullptr) {}
 *     ListNode(int x, ListNode *next) : val(x), next(next) {}
 * };
 */
class Solution {
public:
    ListNode* mergeKLists(vector<ListNode*>& lists) {
        int k = lists.size();
        if(k == 0)
            return NULL;

        int minIndex = -1;
        int minVal = INT_MAX;
        ListNode* retVal;
        for(int i = 0; i < k; i++) {
            if(lists[i]) {
                if(lists[i]->val <= minVal) {
                    minVal = lists[i]->val;
                    minIndex = i;
                }
            }
        }

        if(minIndex == -1)
            return lists[0];

        retVal = lists[minIndex];
        lists[minIndex] = lists[minIndex]->next;
        bool isRunning = true;
        while(isRunning) {
            minIndex = -1;
            minVal = INT_MAX;
            for(int i = 0; i < k; i++) {
                if(lists[i]) {
                    if(lists[i]->val < minVal) {
                        minVal = lists[i]->val;
                        minIndex = i;
                    }
                }
            }

            if(minIndex == -1)
                return retVal;
            
            // TODO this insertion is the wrong way around and we insert too much
            ListNode* temp = new ListNode(lists[minIndex]->val);
            retVal->next = temp;
            retVal = retVal;
            lists[minIndex] = lists[minIndex]->next;
        }

        return retVal;
    }
};