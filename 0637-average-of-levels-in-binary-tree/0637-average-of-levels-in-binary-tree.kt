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

 import java.math.BigDecimal
 import java.math.RoundingMode

class Solution {
    fun dfs(
        target: TreeNode?, 
        level: Int, 
        countList: MutableList<Int>, 
        sumList: MutableList<Double>
    ) {
        if (target == null) {
            return
        }

        if (level + 1 > countList.size) {
            countList.add(0)
            sumList.add(0.00)
        }

        countList[level] += 1
        sumList[level] += target.`val`.toDouble()

        dfs(target.left, level + 1, countList, sumList)
        dfs(target.right, level + 1, countList, sumList)
    }
    fun averageOfLevels(root: TreeNode?): DoubleArray {
        var countList = mutableListOf<Int>()
        var sumList = mutableListOf<Double>()

        dfs(root, 0, countList, sumList)

        return DoubleArray(countList.size)  { level ->
            val nodeCount = countList[level]
            val nodeSum = sumList[level]

            BigDecimal(nodeSum).divide(BigDecimal(nodeCount), 5, RoundingMode.DOWN).toDouble()
        }
    }
}