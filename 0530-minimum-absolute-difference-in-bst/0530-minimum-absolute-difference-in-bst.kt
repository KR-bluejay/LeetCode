/**
 * Example:
 * var ti = TreeNode(5)
 * var v = ti.`val`
 * Definition for a binary tree node.
 * class TreeNode(var `val`: Int) {
 *     var left: TreeNode? = null
 *     var right: TreeNode? = null
 * }
 */
class Solution {
    var prevNode: TreeNode? = null
    var minValue = Int.MAX_VALUE

    fun inOrder(targetNode: TreeNode?) {
        if (targetNode == null) {
            return
        }

        inOrder(targetNode.left)
        
        if (prevNode != null) {
            minValue = Math.min(minValue, abs(prevNode!!.`val` - targetNode.`val`))
        }

        prevNode = targetNode
        inOrder(targetNode.right)
    }
    fun getMinimumDifference(root: TreeNode?): Int {
        if (root == null) {
            return -1
        }

        inOrder(root)


        return minValue
    }
}