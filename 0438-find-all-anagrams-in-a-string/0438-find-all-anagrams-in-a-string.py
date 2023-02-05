class Solution:
    def findAnagrams(self, s: str, p: str) -> List[int]:
        p_count = Counter(p)
        res = []
        for start, end in enumerate(range(len(p), len(s)+1)):
            if Counter(s[start:end]) == p_count:
                res.append(start)
        return res