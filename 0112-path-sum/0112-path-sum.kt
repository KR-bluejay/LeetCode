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
    fun hasPathSum(root: TreeNode?, targetSum: Int): Boolean {
        val nodeQueue: Queue<Pair<TreeNode, Int>> = LinkedList()

        if (root == null) {
            return false
        }

        nodeQueue.add(Pair(root!!, 0))


        while (!nodeQueue.isEmpty()) {
            val (currentNode, parentNodeVal) = nodeQueue.remove()

            if (currentNode.`val` + parentNodeVal == targetSum && currentNode.left == null && currentNode.right == null) {
                return true
            }

            if (currentNode.left != null) {
                nodeQueue.add(Pair(currentNode.left!!, currentNode.`val`!! + parentNodeVal)) 
            }

            if (currentNode.right != null) {
                nodeQueue.add(Pair(currentNode.right!!, currentNode.`val`!! + parentNodeVal))
            }
        }

        return false
    }
}