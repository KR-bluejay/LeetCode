impl Solution {
    pub fn max_sum_div_three(mut nums: Vec<i32>) -> i32 {
        let mut cache: Vec<Vec<i32>> = vec![vec![0; 3]; nums.len()];

        cache[0][(nums[0] % 3) as usize] = nums[0];

        for (num_id, num_val) in nums.iter().enumerate().skip(1) {
            cache[num_id][0] = cache[num_id - 1][0];
            cache[num_id][1] = cache[num_id - 1][1];
            cache[num_id][2] = cache[num_id - 1][2];

            match (cache[num_id - 1][0] + num_val) % 3 {
                0 => cache[num_id][0] = cache[num_id][0].max(cache[num_id - 1][0] + num_val),
                1 => cache[num_id][1] = cache[num_id][1].max(cache[num_id - 1][0] + num_val),
                2 => cache[num_id][2] = cache[num_id][2].max(cache[num_id - 1][0] + num_val),
                _ => {}
            }
            match (cache[num_id - 1][1] + num_val) % 3 {
                0 => cache[num_id][0] = cache[num_id][0].max(cache[num_id - 1][1] + num_val),
                1 => cache[num_id][1] = cache[num_id][1].max(cache[num_id - 1][1] + num_val),
                2 => cache[num_id][2] = cache[num_id][2].max(cache[num_id - 1][1] + num_val),
                _ => {}
            }
            match (cache[num_id - 1][2] + num_val) % 3 {
                0 => cache[num_id][0] = cache[num_id][0].max(cache[num_id - 1][2] + num_val),
                1 => cache[num_id][1] = cache[num_id][1].max(cache[num_id - 1][2] + num_val),
                2 => cache[num_id][2] = cache[num_id][2].max(cache[num_id - 1][2] + num_val),
                _ => {}
            }
        }

        cache[nums.len() - 1][0]
    }
}