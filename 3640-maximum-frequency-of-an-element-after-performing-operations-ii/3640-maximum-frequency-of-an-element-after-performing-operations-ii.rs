#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

impl Solution {
    pub fn max_frequency(mut nums: Vec<i32>, k: i32, num_operations: i32) -> i32 {
        nums.sort();

        let mut num_id: usize = 0;
        let mut num_counts: Vec<i32> = Vec::with_capacity(nums.len() / 2);
        let mut num_values: Vec<i32> = Vec::with_capacity(nums.len() / 2);

        while num_id < nums.len() {
            let num_value = nums[num_id];
            let mut num_count = 1;
            let mut next_num_id = num_id + 1;

            while next_num_id < nums.len() && num_value == nums[next_num_id] {
                num_count += 1;
                next_num_id += 1;
            }

            num_counts.push(num_count);
            num_values.push(num_value);
            num_id = next_num_id;
        }

        let mut max_freq_count = 0;

        let mut base_id = 0;
        for (num_id, num_value) in nums.iter().enumerate() {
            while base_id < nums.len() && 2 * k < num_value - nums[base_id] {
                base_id += 1;
            }

            max_freq_count = max_freq_count.max((num_id as i32 - base_id as i32 + 1)
                .min(num_operations));
        }

        let mut left_id = 0;
        let mut right_id = 0;

        if is_x86_feature_detected!("avx2") && num_values.len() >= 8 {
            unsafe {
                let k_vec = _mm256_set1_epi32(k);
                let num_ops_vec = _mm256_set1_epi32(num_operations);
                let one_vec = _mm256_set1_epi32(1);

                let mut global_max = _mm256_set1_epi32(max_freq_count);

                for chunk_id in (0..num_values.len()).step_by(8) {
                    let chunk_end = (chunk_id + 8).min(num_values.len());
                    let chunk_size = chunk_end - chunk_id;

                    let mut chunk_values = [0i32; 8];
                    let mut chunk_counts = [0i32; 8];

                    for i in 0..chunk_size {
                        chunk_values[i] = num_values[chunk_id + i];
                        chunk_counts[i] = num_counts[chunk_id + i];
                    }

                    for i in chunk_size..8 {
                        chunk_values[i] = chunk_values[chunk_size - 1];
                        chunk_counts[i] = chunk_counts[chunk_size - 1];
                    }

                    let values_vec = _mm256_setr_epi32(
                        chunk_values[0],
                        chunk_values[1],
                        chunk_values[2],
                        chunk_values[3],
                        chunk_values[4],
                        chunk_values[5],
                        chunk_values[6],
                        chunk_values[7],
                    );

                    let counts_vec = _mm256_setr_epi32(
                        chunk_counts[0],
                        chunk_counts[1],
                        chunk_counts[2],
                        chunk_counts[3],
                        chunk_counts[4],
                        chunk_counts[5],
                        chunk_counts[6],
                        chunk_counts[7],
                    );

                    for i in 0..chunk_size {
                        let num_id = chunk_id + i;
                        let num_value = num_values[num_id];

                        while left_id < nums.len() && k < num_value - nums[left_id] {
                            left_id += 1;
                        }

                        while right_id < nums.len() && nums[right_id] - num_value <= k {
                            right_id += 1;
                        }

                        let cur_count = num_counts[num_id] as i32;
                        let window_size = (right_id as i32 - left_id as i32);
                        let add_count = (window_size - cur_count).min(num_operations);

                        max_freq_count = max_freq_count.max(cur_count + add_count);
                    }
                }
                max_freq_count
            }
        } else {
            for (num_id, num_value) in num_values.iter().enumerate() {
                while left_id < nums.len() && k < num_value - nums[left_id] {
                    left_id += 1;
                }

                while right_id < nums.len() && nums[right_id] - num_value <= k {
                    right_id += 1;
                }

                let cur_count = num_counts[num_id] as i32;
                let add_count = ((right_id as i32 - left_id as i32) - cur_count)
                    .min(num_operations);

                max_freq_count = max_freq_count.max(cur_count + add_count);
            }
            max_freq_count
        }
    }
}