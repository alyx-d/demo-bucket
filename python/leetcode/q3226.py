class Solution:
    def minChanges(self, n: int, k: int) -> int:
        if n & k != k:
            return -1

        def countOne(num: int) -> int:
            count = 0
            while num:
                if num & 1 == 1:
                    count += 1
                num >>= 1
            return count

        def countOne(num: int) -> int:
            count = 0
            while num:
                count += 1
                num &= num - 1
            return count

        return countOne(n ^ k)

    def minChanges(self, n: int, k: int) -> int:
        return -1 if n & k != k else (n ^ k).bit_count()
