class Solution {
    public int maxProfit(int[] prices) {
        // sliding window

        // pointer to the smallest value found
        int min_value = prices[0];
        int res = 0;

        for (int i = 0; prices.length > i; i++) {
            // update the minimum value we've seen so far
            min_value = Math.min(prices[i],min_value);

            res = Math.max(prices[i] - min_value, res);
        }

        return res;
    }
}
