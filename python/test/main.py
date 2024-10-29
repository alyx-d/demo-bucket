import unittest
from .. import leetcode

class TestLeetCode(unittest.TestCase):
    def test_1(self):
        s1 = leetcode.q1.Solution()
        res = s1.twoSum([1,2,3], 5)
        self.assertEqual(res, [1,2])

if __name__ == '__main__':
    unittest.main()