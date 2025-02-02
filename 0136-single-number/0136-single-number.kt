class Solution {
    fun singleNumber(nums: IntArray): Int {
        val numMap = nums.toList().groupingBy { it }.eachCount()

        for ((numKey, numCount) in numMap) {
            if (numCount == 1) {
                return numKey
            }
        }

        return -1
    }
}