use std::collections::BTreeSet;

impl Solution {
    pub fn maximize_square_area(
        m: i32, 
        n: i32, 
        mut h_fences: Vec<i32>, 
        mut v_fences: Vec<i32>
    ) -> i32 {
        h_fences.sort_unstable();
        v_fences.sort_unstable();

        if h_fences.binary_search(&1).is_err() {
            h_fences.push(1);
        }
        if h_fences.binary_search(&m).is_err() {
            h_fences.push(m);
        }

        if v_fences.binary_search(&1).is_err() {
            v_fences.push(1);
        }
        if v_fences.binary_search(&n).is_err() {
            v_fences.push(n);
        }

        h_fences.sort();
        v_fences.sort();

        let mut h_fence_set: BTreeSet<i32> = BTreeSet::new();
        let mut v_fence_set: BTreeSet<i32> = BTreeSet::new();

        for i in 0 .. h_fences.len() {
            let i_len = h_fences[i];

            for j in i + 1 .. h_fences.len() {
                let j_len = h_fences[j];

                h_fence_set.insert((i_len - j_len).abs());
            }
        }

        for i in 0 .. v_fences.len() {
            let i_len = v_fences[i];

            for j in i + 1 .. v_fences.len() {
                let j_len = v_fences[j];

                v_fence_set.insert((i_len - j_len).abs());
            }
        }

        let h_fence_set: Vec<i32> = h_fence_set.into_iter().collect();
        let v_fence_set: Vec<i32> = v_fence_set.into_iter().collect();

        let mut row_id = h_fence_set.len() - 1;
        let mut col_id = v_fence_set.len() - 1;


        while row_id < h_fence_set.len() && col_id < v_fence_set.len() {
            if h_fence_set[row_id] == v_fence_set[col_id] {
                return ((h_fence_set[row_id] as usize).pow(2) % 1_000_000_007) as i32
            }

            if h_fence_set[row_id] < v_fence_set[col_id] {
                col_id -= 1;
            } else {
                row_id -= 1;
            }
        }

        -1
    }
}