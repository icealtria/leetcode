class Solution:
    def maxIceCream(self, costs: List[int], coins: int) -> int:
        costs.sort()
        res = 0
        for price in costs:
            coins -= price
            if coins >= 0:
                res+=1
            else:
                break
        return res