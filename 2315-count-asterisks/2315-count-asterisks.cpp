class Solution {
public:
    int countAsterisks(string s) {
        int flag = 0;
        int cnt = 0;
        for (char x:s) {
            if (x == '|') {flag = 1 - flag;}
            if (flag == 0 && x == '*') {++cnt;}
        }
        return cnt;
    }
};