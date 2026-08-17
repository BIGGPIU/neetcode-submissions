# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right

class Solution:
    def maxDepth(self, root: Optional[TreeNode]) -> int:
        # stack = {}
        # stack2 = []
        # pointer = root
        # returnint = 0
        # depth = 1
        # previous = 1

        # # base cases
        # if root == None:
        #     return 0
        # if root.left == None and root.right == None:
        #     return 1

        # while True:
        #     # preform a DFS 
        #     if pointer.left != None:
        #         stack.update({pointer.left:depth + 1})
        #         stack2.append(pointer.left)
        #     if pointer.right != None:
        #         stack.update({pointer.right:depth + 1})
        #         stack2.append(pointer.right)
            
        #     depth += 1
        #     try:
        #         pointer = stack2.pop()
        #     except:
        #         break
        # return max(list(stack.values()))

        if not root:
            return 0 

        stack = [[root,1]]
        res = 1 
        
        while stack:
            node,depth = stack.pop()
            
            if node:
                res = max(res,depth)
                stack.append([node.left, depth + 1])
                stack.append([node.right, depth + 1])
        return res
            