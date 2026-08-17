public class Solution {
    public int[] TwoSum(int[] numbers, int target) {
        int x = 0;
        int y = 1;
        int hold = 0;
        int len =  numbers.Length;

        while (true)
        {
            hold = numbers[x] + numbers[y];
            if (hold == target)
            {
                return new int[]{x+1,y+1};
            }
            else if (hold > target)
            {
                x++;
                y=x+1;
            }
            else
            {
                y++;
                if (y == len)
                {
                    x++;
                    y=x+1;
                }
            }
        }
    }
}