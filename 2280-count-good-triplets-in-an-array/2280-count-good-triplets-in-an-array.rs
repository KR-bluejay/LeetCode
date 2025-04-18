use std::collections::HashMap;


impl Solution {
    fn query(bit: &Vec<i64>, num_id: usize) -> i64 {
        let mut num_id = num_id as i32;
        let mut count: i64 = 0;

        while num_id > 0 {
            count += bit[num_id as usize];
            num_id -= num_id & (num_id * - 1);
        }

        count
    }
    fn update(bit: &mut Vec<i64>, num_id: usize) {
        let mut num_id = num_id as i32;
        let size = bit.len() as i32;

        while num_id < size {
            bit[num_id as usize] += 1;
            num_id += num_id & (num_id * -1);
        }
    }
    pub fn good_triplets(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        let mut bit1: Vec<i64> = vec![0; nums1.len() + 1];
        let mut num2_mapping: HashMap<i32, usize> = HashMap::new();
        let mut res = 0;

        let mut smaller: Vec<i64> = vec![0; nums1.len()];
        let mut larger: Vec<i64> = vec![0; nums1.len()];

        for (num2_id, num_item) in nums2.iter().enumerate() {
            num2_mapping.insert(*num_item, num2_id);
        }

        for i in 0 .. nums1.len() {
            let num2_id = num2_mapping.get(&nums1[i]).unwrap();
            let num2_id_rev = num2_mapping.get(&nums1[nums1.len() - 1 - i]).unwrap();
            
            smaller[*num2_id] = Self::query(&bit1, *num2_id + 1);
            Self::update(&mut bit1, *num2_id + 1);
        }

        std::mem::replace(&mut bit1, vec![0; nums1.len() + 1]);

        for i in (0 .. nums1.len()).rev() {
            let num2_id = num2_mapping.get(&nums1[i]).unwrap();
            let seen = Self::query(&bit1, nums1.len());
            
            larger[*num2_id] = seen - Self::query(&bit1, *num2_id + 1);

            Self::update(&mut bit1, *num2_id + 1);
        }

        for i in 0 .. larger.len() {
            res += larger[i] * smaller[i];
        }

        res
    }
}