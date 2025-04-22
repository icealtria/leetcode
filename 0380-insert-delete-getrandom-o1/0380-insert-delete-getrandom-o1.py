class RandomizedSet:
    def __init__(self):
        self.lst = []
        self.map = {}

    def insert(self, val: int) -> bool:
        if val not in self.map:
            self.map[val] = len(self.lst)
            self.lst.append(val)
            return True
        else:
            return False

    def remove(self, val: int) -> bool:
        if val in self.map:
            idx = self.map[val]
            last = self.lst[-1]
            self.lst[idx] = last
            self.map[last] = idx
            self.lst.pop()
            self.map.pop(val)
            return True
        else:
            return False

    def getRandom(self) -> int:
        if self.lst:
            return random.choice(self.lst)
        else:
            return None



# Your RandomizedSet object will be instantiated and called as such:
# obj = RandomizedSet()
# param_1 = obj.insert(val)
# param_2 = obj.remove(val)
# param_3 = obj.getRandom()