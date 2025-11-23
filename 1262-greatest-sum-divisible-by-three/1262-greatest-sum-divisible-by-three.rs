impl Solution {
    pub fn max_sum_div_three(mut nums: Vec<i32>) -> i32 {
        let mut cache: Vec<Vec<i32>> = vec![vec![0; 3]; nums.len() + 1];

        // cache[0][0] = 0;
        // cache[0][1] = 0;
        // cache[0][1] = 0;

        for (num_id, num_val) in nums.iter().enumerate() {
            let num_id = num_id + 1;

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
        // println!("{cache:?}");
        cache[nums.len()][0]
    }
}