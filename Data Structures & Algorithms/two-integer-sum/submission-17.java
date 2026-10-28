class Solution {
    public int[] twoSum(int[] nums, int target) {
        // Value, Index
        HashMap<Integer,Integer> set = new HashMap();

        for (int i = 0; nums.length > i; i++) {
            set.put(nums[i],i);
        }

        System.out.println(set);
        for (int i = 0; nums.length > i; i++) {
            System.out.println(target - nums[i]);
            Integer value = set.get(target - nums[i]);

            System.out.println(value);

            if (value != null && value != i) {
                return new int[] {i,value.intValue()};
            }
        }


        return new int[] {0,0};
    }
}
