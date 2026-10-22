class Solution:
    def combinationSum(self ,nums: List[int], target: int) -> List[List[int]]:
        res = []
        subset = []

        def dfs(idx,val):
            # print(val)
            # print(subset)

            if val == target:
                res.append(subset.copy())
                return
            if idx >= len(nums) or val > target:
                return

            # what if we stayed with the current decision
            
            # add our decision to our "stack"
            subset.append(nums[idx])
            # then do our decision
            dfs(idx,val + nums[idx])
            # remove our decision from the "stack"
            subset.pop()
            # move on to our next d
            dfs(idx + 1, val)

        dfs(0,0)
        return res