class Solution {
    public int maxProfit(int[] prices) {
        // brute force
        int max_value = 0;


        for (int i = 0; prices.length > i; i++) {
            for (int x = i; prices.length > x; x++) {
                max_value = Math.max(prices[x] - prices[i],max_value);
            }
        }


        return max_value;
    }
}
