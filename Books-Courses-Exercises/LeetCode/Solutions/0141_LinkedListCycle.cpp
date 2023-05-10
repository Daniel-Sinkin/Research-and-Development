/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode(int x) : val(x), next(NULL) {}
 * };
 */
class Solution {
public:
    bool hasCycle(ListNode *head) {
        if(!head)
            return false;
        ListNode* slow = head->next;
        if(!slow)
            return false;
        ListNode* fast = head->next->next;
        if(!fast)
            return false;

        while(slow != fast) {
            slow = slow->next;
            if(!fast->next)
                return false;
            fast = fast->next->next;
            if(!fast)
                return false;
        }

        return true;
    }
};