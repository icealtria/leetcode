class Solution {
public:
    int minFlipsMonoIncr(string s) {
        int cnt0 = 0, cntf = 0;
        for(auto x : s) {
            if (x == '0') cnt0++;
            else cntf++;
            cnt0 = min(cnt0, cntf);
        }
        return cnt0;
    }
};