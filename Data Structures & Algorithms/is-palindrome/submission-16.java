class Solution {
    public boolean isPalindrome(String s) {
        int right_ptr = s.length() - 1;
        int left_ptr = 0;

        if (1 >= s.length()) {
            return true;
        }


        while (right_ptr > left_ptr) {
            char left_value = s.charAt(left_ptr);
            char right_value = s.charAt(right_ptr);

            System.out.println("left: " + left_value + "\n" + "right: " + right_value + "\n");

            if (!Character.isLetterOrDigit(left_value)) {
                left_ptr = left_ptr + 1;
                continue;
            }
            else if (!Character.isLetterOrDigit(right_value)) {
                right_ptr = right_ptr - 1;
                continue;
            }

            if (Character.toLowerCase(left_value) == Character.toLowerCase(right_value)) {
                right_ptr = right_ptr - 1;
                left_ptr = left_ptr + 1;
                continue;
            }
            else {
                return false;
            }
        }


        return true;
    }
}
