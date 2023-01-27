class Solution:
    def findAllConcatenatedWordsInADict(self, words: List[str]) -> List[str]:
        s = set(words)
        res = []
        
        def dfs(word: str) -> bool:
            for i in range(1, len(word)):
                prefix = word[:i]
                suffix = word[i:]
                if prefix in s and (suffix in s or dfs(suffix)):
                    return True
            return False
        
        for word in words:
            if dfs(word):
                res.append(word)
        return res