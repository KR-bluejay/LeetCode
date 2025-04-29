impl Solution {
    fn build_tree(
        seg_tree: &mut Vec<i32>, 
        seg_id: usize,
        start_id: usize, 
        end_id: usize,
        code: &Vec<i32>, 
    ) -> i32 {
        if start_id == end_id {
            seg_tree[seg_id] = code[start_id];

            return seg_tree[seg_id];
        }

        let mid_id = start_id + (end_id - start_id) / 2;
        let left_sum = Self::build_tree(
            seg_tree, 
            seg_id * 2, 
            start_id, 
            mid_id, 
            code
        );
        let right_sum = Self::build_tree(
            seg_tree, 
            seg_id * 2 + 1, 
            mid_id + 1, 
            end_id, 
            code
        );

        seg_tree[seg_id] = left_sum + right_sum;
        seg_tree[seg_id]
    }
    fn query(
        seg_tree: &Vec<i32>, 
        seg_id: usize,
        start_id: usize, 
        end_id: usize, 
        left_id: usize,
        right_id: usize,
    ) -> i32 {
        if end_id < left_id || right_id < start_id {
            return 0;
        }

        if left_id <= start_id && end_id <= right_id {
            return seg_tree[seg_id];
        }

        let mid_id = start_id + (end_id - start_id) / 2;

        let left_sum = Self::query(
            seg_tree, 
            seg_id * 2, 
            start_id, 
            mid_id, 
            left_id, 
            right_id
        );
        let right_sum = Self::query(
            seg_tree, 
            seg_id * 2 + 1, 
            mid_id + 1, 
            end_id, 
            left_id, 
            right_id
        );

        left_sum + right_sum
    }
    pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
        let mut encrypted_code: Vec<i32> = vec![0; code.len()];
        if k == 0 {
            return encrypted_code;
        }

        let seg_len = 1 << u32::BITS - (code.len() - 1).leading_zeros();
        let mut seg_tree: Vec<i32> = vec![0; (seg_len << 1) as usize];
        Self::build_tree(&mut seg_tree, 1, 0, code.len() - 1, &code);

        for i in 0 .. code.len() as i32 {
            let (mut left_id, mut right_id) = if k > 0 {
                (i + 1, i + k)
            } else {
                (i + k + code.len() as i32, i - 1 + code.len() as i32)
            };
            left_id %= code.len() as i32;
            right_id %= code.len() as i32;
            
            let left_id = left_id as usize;
            let right_id = right_id as usize;

            encrypted_code[i as usize] = if left_id <= right_id {
                Self::query(
                    &seg_tree, 
                    1, 
                    0, 
                    code.len() - 1, 
                    left_id, 
                    right_id
                )
            } else {
                Self::query(
                    &seg_tree, 
                    1, 
                    0, 
                    code.len() - 1, 
                    left_id, 
                    code.len() - 1
                ) +
                Self::query(
                    &seg_tree, 
                    1, 
                    0, 
                    code.len() - 1, 
                    0, 
                    right_id
                )
            };
        }

        encrypted_code
    }
}