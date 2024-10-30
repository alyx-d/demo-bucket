from itertools import pairwise


class Solution:
    def getSmallestString(self, s: str) -> str:
        arr = [int(it) for it in list(s)]
        n = len(arr)
        for a in range(n - 1):
            b = a + 1
            if arr[a] > arr[b] and self.bothOddOrEven(arr[a], arr[b]):
                arr[a], arr[b] = arr[b], arr[a]
                return "".join([str(it) for it in arr])
        return "".join([str(it) for it in arr])

    def bothOddOrEven(self, a: int, b: int):
        return a % 2 == b % 2


class Data(object):

    def __init__(self, val: int):
        self._val = val

    @property
    def value(self):
        return self._val


if __name__ == "__main__":
    s = Solution()
    print(s.getSmallestString("10"))
    print(ord("1"))
    d = Data(123)
    print(d.value)
