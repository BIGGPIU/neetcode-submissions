class Solution:
    def search(self, nums: list[int], target: int) -> int:
        left = 0
        right = len(nums)-1
        mid = right//2
        while (right >= left):
            if nums[mid] == target:
                return mid
            elif nums[mid] > target:
                right = mid - 1
                mid = right//2 
            elif nums[mid] < target:
                left = mid + 1
                hold = (right - left)//2
                mid = left + hold
                
        return -1