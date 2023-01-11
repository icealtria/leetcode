# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def addTwoNumbers(self, l1: Optional[ListNode], l2: Optional[ListNode]) -> Optional[ListNode]:
        def toNumber(lst):
            if not lst:
                return 0
            return lst.val + toNumber(lst.next) * 10
        def makeList(num):
            if num == 0:
                return None
            return ListNode(num%10, makeList(num//10))
        res = toNumber(l1) + toNumber(l2)
        return ListNode(0) if res == 0 else makeList(res)