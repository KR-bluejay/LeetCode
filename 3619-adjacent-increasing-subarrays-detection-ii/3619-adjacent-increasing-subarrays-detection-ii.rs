impl Solution {
    pub fn max_increasing_subarrays(nums: Vec<i32>) -> i32 {
        let mut prev_id = 0;
        let mut subarray_lens: Vec<usize> = Vec::with_capacity(nums.len());
        let mut max_subarray: usize = 0;

        for i in 1 .. nums.len() {
            if nums[i - 1] >= nums[i] {
                subarray_lens.push(i - prev_id);
                prev_id = i;
            }
        }

        subarray_lens.push(nums.len() - prev_id);


        for i in 0 ..= subarray_lens.len() - 1 {
            max_subarray = max_subarray.max(subarray_lens[i].min(*subarray_lens.get(i + 1).unwrap_or(&0)));
            max_subarray = max_subarray.max(subarray_lens[i].max(*subarray_lens.get(i + 1).unwrap_or(&0)) / 2);
        }

        max_subarray as i32
    }
}