class Solution:
    def isAlienSorted(self, words: List[str], order: str) -> bool:
        d = {ch: i for i, ch in enumerate(order)}
        
        prev = [d[ch] for ch in words[0]]
        for word in words[1:]:
            curr = [d[ch] for ch in word]
            if curr < prev:
                return False
            prev = curr
            
        return True