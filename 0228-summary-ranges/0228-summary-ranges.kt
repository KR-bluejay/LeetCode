class Solution {
    fun summaryRanges(nums: IntArray): List<String> {
        val rangeList = mutableListOf<String>()

        if (nums.size == 0) {
            return rangeList
        } else if (nums.size == 1) {
            rangeList.add(nums[0].toString()) 

            return rangeList 
        }

        var startNum: Int = nums[0]

        for (i in 1 .. nums.lastIndex) {
            if (nums[i - 1] + 1 == nums[i]) {
                continue
            }

            if (startNum == nums[i - 1]) {
                rangeList.add(nums[i - 1].toString())
            } else {
                rangeList.add(startNum.toString() + "->" + nums[i - 1])
            }
            startNum = nums[i]
        }
    
        if (startNum == nums[nums.lastIndex]) {
            rangeList.add(nums[nums.lastIndex].toString())
        } else {
            rangeList.add(startNum.toString() + "->" + nums[nums.lastIndex])
        }


        return rangeList
    }
}