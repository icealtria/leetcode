class Solution:
    def lengthOfLongestSubstring(self, s: str) -> int:
        res, str_list = 0, []
        
        for i in s:
            if i in str_list:
                str_list = str_list[str_list.index(i)+1:]
            str_list.append(i)
            res = max(res,len(str_list))
            
        return res
    
class Solution2:
    def lengthOfLongestSubstring(self, s: str) -> int:
        seen = {}
        ret = 0
        p = -1
        for i, c in enumerate(s):
            if c in seen and seen[c] > p:
                p = seen[c]
            seen[c] = i
            ret = max(ret, i - p)
        return ret
