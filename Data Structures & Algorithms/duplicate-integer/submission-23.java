class Solution {
    public boolean hasDuplicate(int[] nums) {
        HashSet<Integer> set = new HashSet();

        for (Integer p : nums) {
            if (set.add(p) == false) {
                return true;
            }
        }


        return false;
    }
}