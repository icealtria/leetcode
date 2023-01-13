class Solution:
    def rearrangeCharacters(self, s: str, target: str) -> int:
        return min([s.count(k) // v for k,v in Counter(target).items()])