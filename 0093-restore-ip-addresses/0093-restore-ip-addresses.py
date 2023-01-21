class Solution:
    def restoreIpAddresses(self, s: str) -> List[str]:
        res = []
        if len(s) == 4:
            res.append('.'.join(s))
            return res

        def BT(s, address: list):
            if len(address) == 4:
                if not s:
                    res.append('.'.join(address))
                return

            for i in range(1, 4):
                if i > len(s): continue
                if i > 1 and s[0] == '0':
                    continue
                if int(s[:i]) < 256:
                    BT(s[i:], address+[s[:i]])

        BT(s, [])
        return res