class Solution {
    public int[] twoSum(int[] nums, int target) {
        // <Number, Position>
        HashMap<Integer,Integer> valueMap = new HashMap<>();


        for (int i = 0; nums.length > i; i++) {
            valueMap.put(nums[i],i);
        }



        for (int i = 0; nums.length > i; i++) {
            int v = nums[i];
            Integer val = valueMap.get(target - v);

            if (val == null || val == i) {
                continue;
            }
            else {
                return new int[]{i,val};
            }
        }


        return new int[]{};
    }
}
