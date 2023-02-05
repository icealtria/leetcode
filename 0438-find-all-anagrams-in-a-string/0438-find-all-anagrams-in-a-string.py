class Solution:
    def findAnagrams(self, s: str, p: str) -> List[int]:
        lp = len(p)
        if lp > len(s):
            return []
        p_count = Counter(p)
        s_count = Counter(s[:lp])
        res = [0] if p_count == s_count else []
        for i, c in enumerate(s[lp:], 1):
            s_count[s[i-1]] -= 1
            s_count[c] += 1
            if s_count == p_count:
                res.append(i)
        return res