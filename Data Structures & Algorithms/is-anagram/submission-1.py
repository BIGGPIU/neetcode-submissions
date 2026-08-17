class Solution:
    def isAnagram(self, s: str, t: str) -> bool:
        list1 = []
        list2 = []

        for variable in s:
            list1.append(variable)
        for variable in t:
            list2.append(variable)


        list1.sort()
        list2.sort()

        return list1 == list2