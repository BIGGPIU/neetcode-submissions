from collections import Counter

class Solution:
    def hasDuplicate(self, nums: List[int]) -> bool:
        return list(Counter(nums)) != nums