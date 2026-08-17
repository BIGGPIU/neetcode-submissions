class Solution:
    def threeSum(self, nums: list[int]) -> list[list[int]]:
        nums = list(sorted(nums))
        lp = 0
        rp = len(nums)-1
        mp = 1
        answer = []
        while rp >= mp:
            if rp == mp:
                lp +=1
                mp = lp+1
                rp = len(nums)-1
            hold = nums[lp] + nums[rp] + nums[mp]
            if hold == 0:
                if [nums[lp],nums[mp],nums[rp]] not in answer:
                    answer.append([nums[lp],nums[mp],nums[rp]])
                mp +=1
            elif hold > 0:
                rp += -1
            elif hold < 0:
                mp += 1
        return answer