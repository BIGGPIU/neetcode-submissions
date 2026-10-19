/**
 * Definition for a binary tree node.
 * public class TreeNode {
 *     int val;
 *     TreeNode left;
 *     TreeNode right;
 *     TreeNode() {}
 *     TreeNode(int val) { this.val = val; }
 *     TreeNode(int val, TreeNode left, TreeNode right) {
 *         this.val = val;
 *         this.left = left;
 *         this.right = right;
 *     }
 * }
 */

class Solution {
    public TreeNode invertTree(TreeNode root) {
        if (root == null) {
            return root;
        }
        
        TreeNode head = root;

        // Deque<TreeNode> children = new Deque{root};
        ArrayDeque<TreeNode> children = new ArrayDeque();

        children.addFirst(root);


        while (children.size() != 0) {
            // reverse the left and right 
            TreeNode current = children.removeLast();

            TreeNode temp = current.left;
            current.left = current.right;
            current.right = temp;
            // add the children to the stack
            if (current.right != null) {
                children.addFirst(current.right);
            }
            if (current.left != null) {
                children.addFirst(current.left);
            }
        }

        return head;
    }
}
