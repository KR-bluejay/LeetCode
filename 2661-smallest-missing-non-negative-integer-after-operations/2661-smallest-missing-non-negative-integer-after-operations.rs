use std::collections::{ HashMap, BTreeSet };

impl Solution {
    pub fn find_smallest_integer(nums: Vec<i32>, value: i32) -> i32 {
        let mut modulo_map: HashMap<i32, i32> = HashMap::with_capacity(nums.len());
        let mut num_set: BTreeSet<i32> = BTreeSet::new();

        for num in nums.iter() {
            modulo_map.entry((num % value).rem_euclid(value))
                .and_modify(|v| *v += 1)
                .or_insert(0);
        }

        for (k, &v) in modulo_map.iter() {
            for i in 0 ..= v {
                num_set.insert(k + value * i);
            }
        }


        let mut prev_num = *num_set.first().unwrap();

        if prev_num != 0 {
            return 0;
        }

        for &num in num_set.iter().skip(1) {
            if prev_num + 1 != num {
                return prev_num + 1;
            }
            prev_num = num;
        }
        prev_num + 1
    }
}