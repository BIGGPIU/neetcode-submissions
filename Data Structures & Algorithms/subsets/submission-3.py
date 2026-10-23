class Solution:
    def subsets(self, nums: List[int]) -> List[List[int]]:
        # result
        res = []
        # temp
        subset = []

        # recursive function
        def dfs(i):
            # define base case
            if i >= len(nums):
                # print(i)
                print(subset)
                res.append(subset.copy())
                return

            # make a decision
            subset.append(nums[i])
            # act with that decision
            dfs(i + 1)
            subset.pop()
            dfs(i + 1)

        dfs(0)
        return res