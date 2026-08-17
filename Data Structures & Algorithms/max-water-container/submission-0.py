class Solution:
    def maxArea(self, heights: list[int]) -> int:
        l = 0
        r = 1
        maximum = []
        while l != (len(heights) - 1):
            if r == l+1:
                maximum.append(-99)
            x = min(heights[l],heights[r])
            amnt = x * (r-l)
            maximum[l] = max(amnt,maximum[l])
            r+=1
            if r == len(heights):
                l+=1
                r = l+1
        maximum.sort(reverse=True)
        return maximum[0]