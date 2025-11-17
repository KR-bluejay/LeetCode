impl Solution {
    pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
        let mut prev_id: Option<usize> = None;
        let k = k as usize;

        for (id, num) in nums.into_iter().enumerate() {
            if num == 1 {
                match prev_id {
                    Some(pid) => {
                        if id - pid <= k {
                            return false;
                        }
                        prev_id = Some(id);
                    },
                    None => {
                        prev_id = Some(id);
                    }
                }
            }
        }
        true
    }
}