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
    ListNode* insert(ListNode* header, int data) {
        header->next = new ListNode(data);
        return header->next;
    }

    ListNode* mergeTwoLists(ListNode* list1, ListNode* list2) {
        ListNode * curr = new ListNode(0);
        ListNode * retVal = curr;

        if(!list1)
            return list2;
        if(!list2)
            return list1;

        while(list1 && list2) {
            if(list1->val <= list2->val) {
                curr = insert(curr, list1->val);
                list1 = list1->next;
            } else {
                curr = insert(curr, list2->val);
                list2 = list2->next;
            }
        }

        if(!list1 && list2)
            curr->next = list2;

        if(list1 && !list2)
            curr->next = list1;

        return retVal->next;
    }
};