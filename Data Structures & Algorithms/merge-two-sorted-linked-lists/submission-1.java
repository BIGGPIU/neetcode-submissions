/**
 * Definition for singly-linked list.
 * public class ListNode {
 *     int val;
 *     ListNode next;
 *     ListNode() {}
 *     ListNode(int val) { this.val = val; }
 *     ListNode(int val, ListNode next) { this.val = val; this.next = next; }
 * }
 */

class Solution {
    public ListNode mergeTwoLists(ListNode list1, ListNode list2) {
        ListNode current;
        ListNode head;



        ListNode l = list1;
        ListNode r = list2;

        if (list1 == null) {
            return list2;
        }
        else if (list2 == null) {
            return list1;
        }

        if (list1.val >= list2.val) {
            // because list2 is smaller its going to come first
            head = new ListNode(list2.val);
            r = list2.next;
        }
        else {
            head = new ListNode(list1.val);
            l = list1.next;
        }
        
        current = head;

        // 
        while (l != null || r != null) {
            // try to deplete the right one 
            if (l == null) {
                current.next = r;
                break;
            }
            else if (r == null) {
                current.next = l;
                break;
            }
            else {
                if (l.val >= r.val) {
                    current.next = new ListNode(r.val);
                    r = r.next;
                }
                else {
                    current.next = new ListNode(l.val);
                    l = l.next;
                }
            }

            current = current.next;
        }


        return head;
    }
}