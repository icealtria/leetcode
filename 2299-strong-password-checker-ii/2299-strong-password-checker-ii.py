class Solution:
    def strongPasswordCheckerII(self, password: str) -> bool:
        return re.match(r'^(?!.*(.)\1)(?=.*[a-z])(?=.*[A-Z])(?=.*[0-9])(?=.*[!@#$%^&*()+-]).{8,}$', password)