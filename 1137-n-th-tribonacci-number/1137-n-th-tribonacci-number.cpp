class Solution {
public:
    int dp[38]{0,1,1};
    int tribonacci(int n) {
        if (n==0) return 0;
        if (dp[n]) return dp[n];
        else return dp[n] = tribonacci(n-1) + tribonacci(n-2) + tribonacci(n-3);
    }
};