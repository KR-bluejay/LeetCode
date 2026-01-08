impl Solution {
    pub fn max_dot_product(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut cache = vec![vec![i32::MIN / 2; nums2.len()]; nums1.len()];

        for (num1_id, num1_val) in nums1.into_iter().enumerate() {
            for (num2_id, &num2_val) in nums2.iter().enumerate() {
                let cur_prod = num1_val * num2_val;
                let mut result = cur_prod;

                if num1_id > 0 && num2_id > 0 {
                    result = result.max(cur_prod + cache[num1_id - 1][num2_id - 1].max(0));
                }

                if num1_id > 0 {
                    result = result.max(cache[num1_id - 1][num2_id]);
                }
                if num2_id > 0 {
                    result = result.max(cache[num1_id][num2_id - 1]);
                }
                cache[num1_id][num2_id] = result;
            }
        }

        *cache.last().unwrap().last().unwrap()
    }
}