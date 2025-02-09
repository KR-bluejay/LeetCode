class Solution {

    fun rob(nums: IntArray): Int {
        val houseMap = HashMap<Int, Int>(nums.size)
        var maxProfit = 0

        if (nums.size <= 2) {
            return nums.max()
        }

        for (i in 0 .. 1) {
            maxProfit = Math.max(maxProfit, nums.get(i))
            houseMap.put(i, nums.get(i))
        }

        for (i in 2 .. nums.size - 1) {
            val moneySum = Math.max(houseMap.getOrDefault(i - 2, 0), houseMap.getOrDefault(i - 3, 0)) + nums.get(i)

            houseMap.put(i, moneySum)

            maxProfit = Math.max(moneySum, maxProfit)
        }

        return maxProfit
    }
}