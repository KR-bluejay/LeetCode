class Solution {
    fun twoSum(nums: IntArray, target: Int): IntArray {
        val numMap = HashMap<Int, Int>()

        for ((numIndex, numItem) in nums.withIndex()) {
            val pairNumIndex: Int = numMap.getOrDefault(target - numItem, -1)

            if (pairNumIndex != -1) {
                return intArrayOf(numIndex, pairNumIndex)
            }

            numMap.put(numItem, numIndex)
        }

        return IntArray(2)

    }
}