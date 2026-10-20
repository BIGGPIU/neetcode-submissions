class Solution {
    public int findDuplicate(int[] nums) {
        // Solution 2
        // Sort the array and then just check the element in front
        // Space: O(1)
        // Time: O(n * log(n))

        Arrays.sort(nums);

        for (int i = 0; nums.length > i; i++) {
            if (nums[i + 1] == nums[i]) {
                return nums[i];
            }
        }


        return -1;
    }
}
