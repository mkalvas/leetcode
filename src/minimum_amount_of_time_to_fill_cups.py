# id=2335 slug=minimum-amount-of-time-to-fill-cups lang=python3

class Solution:
    def fillCups(self, amount: list[int]) -> int:
        amount.sort()
        return max(amount[2], ((sum(amount) + 1) // 2))


if __name__ == "__main__":
    import unittest

    class TestFillCups(unittest.TestCase):
        def test_base_1(self):
            self.assertEqual(Solution().fillCups([1, 4, 2]), 4)

        def test_base_2(self):
            self.assertEqual(Solution().fillCups([5, 4, 4]), 7)

        def test_base_3(self):
            self.assertEqual(Solution().fillCups([5, 0, 0]), 5)

    unittest.main()
