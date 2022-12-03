class Solution:
    def frequencySort(self, s: str) -> str:
        return ''.join(ch * c for ch, c in collections.Counter(s).most_common())