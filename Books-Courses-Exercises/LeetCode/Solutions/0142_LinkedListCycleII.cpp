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
    ListNode* detectCycle(ListNode *head) {
        ListNode* slow = head;
        ListNode* fast = head;

        ListNode* iter = head;

        int counter = 0;
        int cycle = 0;

        vector<ListNode*> cycleNodes;

        while(fast) {
            counter++;
            slow = slow->next;
            fast = fast->next;
            if(!fast)
                return NULL;
            fast = fast->next;

            if(slow == NULL || fast == NULL)
                return NULL;

            // If slow and fast meet then there had to be a cycle
            if(slow == fast) {
                slow = slow->next;
                fast = fast->next->next;
                cycle = 1;

                // Determines the cycle length
                while(slow != fast) {
                    slow = slow->next;
                    fast = fast->next->next;
                    cycle++;
                }

                cout << "Size of Cycle = "<< cycle << "\n";

                for(int i = 0; i < cycle; i++) {
                    cycleNodes.push_back(slow);
                    slow = slow->next;
                }

                //for(int r = 0; r < cycle; r++)
                //    printf("The %d. value of the cycle is = %d\n", r+1, cycleNodes[r]);

                while(true) {
                    // Checks if iter is in the cycle
                    for(int i = 0; i < cycle; i++) {
                        if(cycleNodes[i] == iter)
                            return iter;
                    }

                    iter = iter->next;
                }
            }
        }

        return NULL;
    }
};