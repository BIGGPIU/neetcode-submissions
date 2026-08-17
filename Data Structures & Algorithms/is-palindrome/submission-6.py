class Solution:
    def isPalindrome(self, s: str) -> bool:
        s = s.replace(" ","")
        s = s.replace("?","")
        s = s.replace(".","")
        s = s.replace(",","")
        s = s.replace("'","")
        s = s.replace(":","")
        s = s.lower()
        # x = helloworld
        # x[0]
        # x[-1]
        # x[var] == x[-var]
        for variable in range(len(s)):
            hold = s[variable]
            hold2 = s[-variable-1]
            if s[variable] == s[-variable-1]:
                pass
            else:
                return False
        
        return True
        