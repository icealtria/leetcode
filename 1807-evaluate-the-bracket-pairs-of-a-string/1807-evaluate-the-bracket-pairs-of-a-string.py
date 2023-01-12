class Solution:
    def evaluate(self, s: str, knowledge: List[List[str]]) -> str:
        d = {k: v for k, v in knowledge}
        t = s.split('(')
        res = t[0]
        for s in t[1:]:
            x, y = s.split(')')
            res += d.get(x, '?') + y
        return res