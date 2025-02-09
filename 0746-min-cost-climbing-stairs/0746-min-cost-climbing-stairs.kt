class Solution {
    fun minCostClimbingStairs(cost: IntArray): Int {
        for (i in 2 .. cost.lastIndex) {
            cost[i] = Math.min(cost[i - 1], cost[i - 2]) + cost[i]
        }


        return Math.min(cost.get(cost.lastIndex), cost.get(cost.lastIndex - 1))
    }
}