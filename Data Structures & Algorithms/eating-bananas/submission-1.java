class Solution {
    public int minEatingSpeed(int[] piles, int h) {
        // Solution 2 
        int maxBananas = 0;

        for (int i : piles) {
            maxBananas = Math.max(maxBananas,i);
        }


        // 1 is the min amount of banananas we can eat
        int left_ptr = 1;
        // the most bananas we *should* eat at once is maxBananas
        int right_ptr = maxBananas;
        int result = right_ptr;

        while (right_ptr >= left_ptr) {
            int time_to_eat = 0;
            int center_ptr = (right_ptr + left_ptr) / 2;

            for (int i : piles) {
                time_to_eat += Math.ceil((double) i / center_ptr);
            }

            // if the time to eat is too small then move the left bound
            if (h >= time_to_eat) {
                result = center_ptr;
                right_ptr = center_ptr - 1;
            }
            else {
                // move the right pointer anyways
                left_ptr = center_ptr + 1;
            }

        }

        return result;
    }
}
