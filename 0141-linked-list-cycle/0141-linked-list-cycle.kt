/**
 * Example:
 * var li = ListNode(5)
 * var v = li.`val`
 * Definition for singly-linked list.
 * class ListNode(var `val`: Int) {
 *     var next: ListNode? = null
 * }
 */

class Solution {
    fun hasCycle(head: ListNode?): Boolean {
        var slowNode: ListNode? = head;
        var fastNode: ListNode? = head?.next;


        while (slowNode != null && fastNode != null)  {
            if (slowNode === fastNode) {
                return true
            }


            slowNode = slowNode?.next
            fastNode = fastNode?.next?.next
        }

        return false
    }
}