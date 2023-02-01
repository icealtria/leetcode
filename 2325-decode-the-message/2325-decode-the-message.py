class Solution:
    def decodeMessage(self, key: str, message: str) -> str:
        letters = 'abcdefghijklmnopqrstuvwxyz'
        d = {' ': ' '}
        i = 0
        for k in key:
            if k not in d:
                d[k] = letters[i]
                i += 1
            if i > 26:
                break
        res = []
        for m in message:
            res.append(d[m])
        return ''.join(res)