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
    ListNode* reverseList(ListNode* head) {
        // On empty we return empty, 1-elems are preserved under reverse
        if(!head || !head->next)
            return head;

        ListNode* tail = new ListNode(head->val);
        ListNode* retVal = new ListNode(head->next->val, tail);
        head = head->next->next;

        ListNode* temp;
        while(head) {
            temp = new ListNode(head->val, retVal);
            retVal = temp;
            head = head->next;
        }

        return retVal;
    }
};