class Solution {
    public int findDuplicate(int[] nums) {
        // Solution 1: The really stupid one
        // Space: Constant
        // Time: O(n^2)


        for (int i = 0; nums.length > i; i++) {
            for (int x = 0; nums.length > x; x++) {
                if (i == x) {
                    continue;
                }
                else {
                    if (nums[i] == nums[x]) {
                        return nums[i];
                    }
                }
            }
        }


        return -1;
    }
}
