class Solution:
    def plusOne(self, digits: list[int]) -> list[int]:
        answer = 0
        for i in range(len(digits)):
            hold2 = i+1
            hold = len(digits)-hold2
            answer += digits[i]*10**(hold)
        answer += 1
        answer = str(answer)
        answerlist = []
        for i in answer:
            answerlist.append(i)
        return answerlist