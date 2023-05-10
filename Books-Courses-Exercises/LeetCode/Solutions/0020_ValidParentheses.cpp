class Solution {
public:
    bool isValid(string s) {
        stack<char> stack;

        char temp = ' ';
        for(int i = 0; i < s.size(); i++) {
            if(s[i] == '(' || s[i] == '[' || s[i] == '{') {
                stack.push(s[i]);
            } else {
                if(stack.empty())
                    return false;
                else {
                    temp = stack.top();
                    if(s[i] == ')' && temp != '(' || s[i] == ']' && temp != '[' || s[i] == '}' && temp != '{')
                        return false;
                    stack.pop();
                }
            }
        }
        
        return stack.empty();
    }
};