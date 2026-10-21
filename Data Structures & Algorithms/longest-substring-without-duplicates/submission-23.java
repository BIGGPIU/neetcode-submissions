class Solution {
        // stolen from neetcode
    public int lengthOfLongestSubstring(String s) {
        HashSet<Character> set = new HashSet();
        int left_ptr = 0;
        int res = 0;


        for (int right_ptr = 0; s.length() > right_ptr; right_ptr++) {
            while (set.contains(s.charAt(right_ptr))) {
                set.remove(s.charAt(left_ptr));
                left_ptr += 1;
            }

            set.add(s.charAt(right_ptr));
            res = Math.max(res, right_ptr - left_ptr + 1);
        }

        return res;
    }
}
