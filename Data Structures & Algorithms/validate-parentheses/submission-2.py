dic = {
    "[":"]",
    "{":"}",
    "(":")"
}

class Solution:
    def isValid(self, s: str) -> bool:
        stack = []
        for i in s:
            if i in dic.keys():
                stack.append(i)
            else:
                try:
                    hold = stack[-1]
                    if i == dic[hold]:
                        stack.pop()
                    else:
                        return False
                except:
                    return False
        return len(stack) == 0

