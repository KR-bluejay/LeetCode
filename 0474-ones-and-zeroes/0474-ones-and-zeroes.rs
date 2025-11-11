impl Solution {
    fn find(
        id: usize,
        num_counts: &Vec<(usize, usize)>,
        cache: &mut Vec<Vec<Vec<i32>>>,
        prev_zero_count: usize,
        prev_one_count: usize,
        form_count: i32,
        max_zero_count: usize,
        max_one_count: usize,
        max_form_count: &mut i32,
    ) {
        if id == num_counts.len() {
            return;
        }

        let (zero_count, one_count) = 
            (num_counts[id].0 + prev_zero_count
            , num_counts[id].1 + prev_one_count);

        if cache[id][prev_zero_count][prev_one_count] < form_count {
            cache[id][prev_zero_count][prev_one_count] = form_count;
            *max_form_count = (*max_form_count).max(form_count);

            Self::find(
                id + 1, 
                num_counts, 
                cache, 
                prev_zero_count, 
                prev_one_count, 
                form_count, 
                max_zero_count, 
                max_one_count,
                max_form_count
            );
        }

        if zero_count <= max_zero_count 
        && one_count <= max_one_count 
        && cache[id][zero_count][one_count] < form_count + 1 {
            cache[id][zero_count][one_count] = form_count + 1;
            *max_form_count = (*max_form_count).max(form_count + 1);

            Self::find(
                id + 1, 
                num_counts, 
                cache, 
                zero_count, 
                one_count, 
                form_count + 1, 
                max_zero_count, 
                max_one_count,
                max_form_count
            );
        }
    }
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let mut num_counts: Vec<(usize, usize)> = Vec::with_capacity(strs.len());
        let mut cache: Vec<Vec<Vec<i32>>> 
            = vec![vec![vec![-1; n as usize + 1]; m as usize + 1]; strs.len()];
        let mut max_form_count = 0;

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
            0, 
            m as usize, 
            n as usize, 
            &mut max_form_count
        );
        max_form_count
    }
}