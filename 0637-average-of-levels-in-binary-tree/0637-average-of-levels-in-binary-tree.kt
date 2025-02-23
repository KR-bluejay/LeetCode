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

 import java.math.BigDecimal;
 import java.math.RoundingMode;


class Solution {
    fun averageOfLevels(root: TreeNode?): DoubleArray {
        val levelNodeMap = mutableMapOf<Int, Pair<Int, Long>>()
        val nodeQueue= LinkedList<Pair<TreeNode?, Int>>()

        nodeQueue.add(Pair(root, 0))

        while (!nodeQueue.isEmpty()) {
            val (currentNode, currentLevel) = nodeQueue.remove()

            if (currentNode == null) {
                continue
            }

            val (currentCount, currentSum) = levelNodeMap.getOrPut(currentLevel) {Pair(0, 0)}

            
            levelNodeMap.put(currentLevel, Pair(currentCount + 1, currentSum + currentNode.`val`))

            println(currentSum)

            nodeQueue.add(Pair(currentNode.left, currentLevel + 1))
            nodeQueue.add(Pair(currentNode.right, currentLevel + 1))
        }



        return DoubleArray(levelNodeMap.size) { level ->
            val (nodeCount, nodeSum) = levelNodeMap[level]!!

            BigDecimal(nodeSum).divide(BigDecimal(nodeCount), 5, RoundingMode.DOWN).toDouble()
        }
    }
}