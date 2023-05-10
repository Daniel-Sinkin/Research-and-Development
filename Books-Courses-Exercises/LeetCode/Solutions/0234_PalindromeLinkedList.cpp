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
    bool isPalindrome(ListNode* head) {
        stack<int> myStack;
        ListNode* currNode = head;

        // If the list is empty then it's certainly a palindrome
        if(currNode == nullptr)
            return true;
        
        int len = 0;

        while(currNode != nullptr) {
            len++;
            currNode = currNode -> next;
        }

        currNode = head;
        for(int i = 0; i < len / 2; i++) {
            myStack.push(currNode -> val);
            currNode = currNode -> next;
        }

        // For odd-element sets we can skip the middle element
        if(len % 2 == 1)
            currNode = currNode -> next;

        while(!myStack.empty()) {
            if(myStack.top() != (currNode -> val))
                return false;

            myStack.pop();
            currNode = currNode -> next;
        }

        return true;
    }
};