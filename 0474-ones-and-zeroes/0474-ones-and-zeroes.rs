impl Solution {
    fn find(
        id: usize,
        num_counts: &Vec<(usize, usize)>,
        cache: &mut Vec<Vec<Vec<i32>>>,
        zero_used: usize,
        one_used: usize,
        max_zero_count: usize,
        max_one_count: usize,
    ) -> i32 {
        if id == num_counts.len() {
            return 0;
        }

        if cache[id][zero_used][one_used] != -1 {
            return cache[id][zero_used][one_used];
        }

        let mut result = Self::find(
            id + 1, 
            num_counts, 
            cache, 
            zero_used, 
            one_used, 
            max_zero_count, 
            max_one_count,
        );

        let (zero_count, one_count) = 
            (num_counts[id].0 + zero_used
            , num_counts[id].1 + one_used);

        if zero_count <= max_zero_count 
        && one_count <= max_one_count {
            let next_result = if id + 1 >= num_counts.len() {
                0
            } else if cache[id + 1][zero_count][one_count] != -1 {
                cache[id + 1][zero_count][one_count]
            } else {
                Self::find(
                    id + 1, 
                    num_counts, 
                    cache, 
                    zero_count, 
                    one_count, 
                    max_zero_count, 
                    max_one_count,
                )
            } + 1;

            result = result.max(next_result);
        }

        cache[id][zero_used][one_used] = result;
        cache[id][zero_used][one_used]
    }
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let mut num_counts: Vec<(usize, usize)> = Vec::with_capacity(strs.len());
        let mut cache: Vec<Vec<Vec<i32>>> 
            = vec![vec![vec![-1; n as usize + 1]; m as usize + 1]; strs.len()];

        for str_item in strs.into_iter() {
            let (zero_count, one_count) = str_item.into_bytes()
                .into_iter()
                .fold((0, 0), |mut acc, val| {
                    if val == b'0' {
                        acc.0 += 1;
                    } else {
                        acc.1 += 1;
                    }
                    acc
                });

            num_counts.push((zero_count, one_count));
        }
        Self::find(
            0, 
            &num_counts, 
            &mut cache, 
            0, 
            0, 
            m as usize, 
            n as usize, 
        )
    }
}