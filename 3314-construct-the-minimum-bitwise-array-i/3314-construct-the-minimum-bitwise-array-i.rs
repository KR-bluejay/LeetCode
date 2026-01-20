impl Solution {
    pub fn min_bitwise_array(nums: Vec<i32>) -> Vec<i32> {
        let mut results: Vec<i32> = Vec::with_capacity(nums.len());

        for (id, num) in nums.into_iter().enumerate() {
            for i in 1 .. num {
                if (i | (i + 1)) == num {
                    results.push(i);
                    
                    break;
                }
            }

            if results.len() == id {
                results.push(-1);
            }
        }

        results
    }
}