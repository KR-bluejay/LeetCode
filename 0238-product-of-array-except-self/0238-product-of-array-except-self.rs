impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut results: Vec<i32> = Vec::with_capacity(nums.len());
        let mut cur = 1;

        for id in 0 .. nums.len() {
            results.push(cur);
            cur *= nums[id];
        }
        cur = 1;
        for id in (0 .. nums.len()).rev() {
            results[id] *= cur;
            cur *= nums[id];
        }

        results
    }
}