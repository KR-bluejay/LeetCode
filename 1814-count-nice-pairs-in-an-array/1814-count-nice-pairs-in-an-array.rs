use std::collections::HashMap;

impl Solution {
    pub fn count_nice_pairs(mut nums: Vec<i32>) -> i32 {
        const MODULO: i64 = 1_000_000_007;

        
        let mut result: i64 = 0;
        let mut num_map: HashMap<i32, i64> = HashMap::with_capacity(nums.len());;

        for num in nums.into_iter() {
            let mut origin_num = num;
            let mut rev_num = 0;

            while origin_num > 0 {
                rev_num = rev_num * 10 + (origin_num % 10);
                origin_num /= 10;
            }


            num_map.entry(num - rev_num).and_modify(|v| {
                result += *v;

                *v += 1;
            }).or_insert(1);

        }




        (result % MODULO) as i32
    }
}