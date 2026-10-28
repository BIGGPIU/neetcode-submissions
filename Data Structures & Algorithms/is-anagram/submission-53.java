class Solution {
    public boolean isAnagram(String s, String t) {
        HashMap<Character,Integer> map = new HashMap();


        for (int i = 0; s.length() > i; i++) {
            char c = s.charAt(i);

            Integer value = map.getOrDefault(c,0);

            map.put(c,value+1);
        }


        for (int i = 0; t.length() > i; i++) {
            char c = t.charAt(i);
            Integer value = map.get(c);

            if (value == null) {
                return false;
            }

            if (value - 1 == 0) {
                map.remove(c);
            }
            else {
                map.put(c, value - 1);
            }
        }


        return map.size() == 0;
    }
}
