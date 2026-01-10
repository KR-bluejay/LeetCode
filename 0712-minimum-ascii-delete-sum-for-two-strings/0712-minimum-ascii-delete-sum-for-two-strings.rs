impl Solution {
    fn get_min_sum(
        s1: &Vec<u8>,
        s2: &Vec<u8>,
        s1_id: usize,
        s2_id: usize,
        cache: &mut Vec<Vec<i32>>,
    ) -> i32 {
        if s1_id == s1.len() && s2_id == s2.len() {
            return 0;
        }

        if s1_id == s1.len() {
            let s2_sum = s2[s2_id .. ].iter().map(|&v| v as i32).sum();

            return s2_sum;
        } else if s2_id == s2.len() {
            let s1_sum = s1[s1_id .. ].iter().map(|&v| v as i32).sum();

            return s1_sum;
        }
        
        if cache[s1_id][s2_id] < i32::MAX / 2 {
            return cache[s1_id][s2_id];
        }


        let mut result = Self::get_min_sum(
            s1,
            s2,
            s1_id + 1,
            s2_id + 1,
            cache,
        ) + if s1[s1_id] == s2[s2_id] {
            0
        } else {
            s1[s1_id] as i32 + s2[s2_id] as i32
        };
        
        result = result.min(Self::get_min_sum(
            s1,
            s2,
            s1_id + 1,
            s2_id,
            cache,
        ) + s1[s1_id] as i32);
        result = result.min(Self::get_min_sum(
            s1,
            s2,
            s1_id,
            s2_id + 1,
            cache,
        ) + s2[s2_id] as i32);

        cache[s1_id][s2_id] = result;
        cache[s1_id][s2_id]
    }
    pub fn minimum_delete_sum(s1: String, s2: String) -> i32 {
        let s1: Vec<u8> = s1.into_bytes();
        let s2: Vec<u8> = s2.into_bytes();

        let mut cache: Vec<Vec<i32>> = vec![vec![i32::MAX / 2; s2.len()]; s1.len()];

        Self::get_min_sum(
            &s1,
            &s2, 
            0,
            0,
            &mut cache
        )
    }
}