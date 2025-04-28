class Solution:
    def canConstruct(self, ransomNote: str, magazine: str) -> bool:
        s1, s2 = Counter(ransomNote), Counter(magazine)
        return s1 & s2 == s1