class Solution:
    def partition(self, s: str) -> List[List[str]]:
        def BT(s: str, parts):
            if not s:
                res.append(parts)
                return
            for i in range(1, len(s)+1):
                part = s[:i]
                if part == part[::-1]:
                    BT(s[i:], parts + [part])
                else:
                    continue

        res = []
        BT(s, [])
        return res