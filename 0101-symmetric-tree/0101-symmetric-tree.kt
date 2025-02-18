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
    fun checkNode(leftNode: TreeNode?, rightNode: TreeNode?): Boolean {
        if (leftNode == null || rightNode == null) {
            return leftNode == rightNode
        }
        
        if (leftNode?.`val` != rightNode?.`val`) {
            return false
        }

        return checkNode(leftNode?.left, rightNode?.right) 
        && checkNode(leftNode?.right, rightNode?.left)
    }
    fun isSymmetric(root: TreeNode?): Boolean {
        var leftNode: TreeNode? = root?.left    
        var rightNode: TreeNode? = root?.right

        return checkNode(leftNode, rightNode)
    }
}