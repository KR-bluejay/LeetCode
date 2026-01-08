impl Solution {
    fn find_prod(
        nums1: &Vec<i32>,
        nums2: &Vec<i32>,
        num_cache: &mut Vec<Vec<i32>>,
        nums1_id: usize,
        nums2_id: usize,
        nums1_count: usize,
        nums2_count: usize,
    ) -> i32 {
        if nums1_id == nums1.len() || nums2_id == nums2.len() {
            return i32::MIN / 2;
        }

        if num_cache[nums1_id][nums2_id] > i32::MIN {
            return num_cache[nums1_id][nums2_id];
        }

        let cur_prod = nums1[nums1_id] * nums2[nums2_id];
        let tmp_a = Self::find_prod(
            nums1,
            nums2,
            num_cache,
            nums1_id + 1, 
            nums2_id + 1,
            nums1_count + 1,
            nums2_count + 1
        );
        let tmp_a = cur_prod.max(cur_prod + tmp_a);
        let mut tmp_b = Self::find_prod(
            nums1,
            nums2,
            num_cache,
            nums1_id + 1, 
            nums2_id,
            nums1_count,
            nums2_count
        );
        let mut tmp_c = Self::find_prod(
            nums1,
            nums2,
            num_cache,
            nums1_id, 
            nums2_id + 1,
            nums1_count,
            nums2_count
        );

        if nums1_count == nums2_count {
            num_cache[nums1_id][nums2_id] = num_cache[nums1_id][nums2_id]
                .max(tmp_a).max(tmp_b).max(tmp_c);
        }

        num_cache[nums1_id][nums2_id]
    }
    pub fn max_dot_product(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut num_cache = vec![vec![i32::MIN; nums2.len()]; nums1.len()];

        Self::find_prod(&nums1, &nums2, &mut num_cache, 0, 0, 0, 0)
    }
}