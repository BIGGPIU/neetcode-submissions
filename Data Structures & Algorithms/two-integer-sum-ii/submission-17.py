class Solution:
    def twoSum(self, numbers: list[int], target: int) -> list[int]:
        x = 0 
        y = 0
        while True:
            hold = numbers[x] + numbers[y]
            if hold == target:
                return [x+1,y+1]
            elif hold> target:
                x+=1
                y=x+1
            else:
                y+=1
                if y == len(numbers):
                    x+=1
                    y=x+1