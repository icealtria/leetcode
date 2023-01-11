class Solution:
    def digitCount(self, num: str) -> bool:
        freq = Counter(num)
        for i, d in enumerate(num):
            if freq[str(i)] != int(d):
                return False
        return True