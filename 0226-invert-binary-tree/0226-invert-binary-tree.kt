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
    fun swapNode(oldTemp: TreeNode?, newTemp: TreeNode?) {
        if (oldTemp == null) {
            return
        }

        newTemp?.left = oldTemp?.right.also{newTemp?.right = oldTemp?.left} 

        swapNode(oldTemp.left, oldTemp.left)
        swapNode(oldTemp.right, oldTemp.right)
    }
    fun invertTree(root: TreeNode?): TreeNode? {
        val newRoot: TreeNode? = root

        var oldTemp: TreeNode? = root
        var newTemp: TreeNode? = newRoot

        swapNode(oldTemp, newTemp)

        return newRoot
    }
}