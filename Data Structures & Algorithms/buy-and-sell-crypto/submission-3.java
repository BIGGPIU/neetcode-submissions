class Solution {
    public int maxProfit(int[] prices) {
        // sliding window

        // pointer to the smallest value found
        int min_value = prices[0];
        int res = 0;

        for (int i = 0; prices.length > i; i++) {
            // update the minimum value we've seen so far
            if (min_value > prices[i]) {
                min_value = prices[i];
            }

            res = Math.max(prices[i] - min_value, res);
        }

        return res;
    }
}
