class Solution:
    def lengthOfLongestSubstring(self, s: str) -> int:
        res, str_list = 0, []
        
        for i in s:
            if i in str_list:
                str_list = str_list[str_list.index(i)+1:]
            str_list.append(i)
            res = max(res,len(str_list))
            
        return res