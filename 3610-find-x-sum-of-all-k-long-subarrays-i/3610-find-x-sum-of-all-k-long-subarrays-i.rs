use std::collections::BTreeSet;

impl Solution {
    #[inline(always)]
    fn current_x_sum(set: &BTreeSet<(i32, i32)>, x: usize) -> i32 {
        let mut sum = 0;
        for &(neg_f, neg_v) in set.iter().take(x) {
            let f = -neg_f;
            let v = -neg_v;
            sum += f * v;
        }
        sum
    }
    #[inline(always)]
    pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let k = k as usize;
        let x = x as usize;
        let n = nums.len();

        let mut freq = [0_i32; 51];

        for &v in nums.iter().take(k) {
            freq[v as usize] += 1;
        }

        let mut set: BTreeSet<(i32, i32)> = BTreeSet::new();
        for value in 1..=50 {
            let f = freq[value];
            if f > 0 {
                set.insert((-f, -(value as i32)));
            }
        }

        

        let mut ans = Vec::with_capacity(n - k + 1);

        ans.push(Self::current_x_sum(&set, x));

        for i in k..n {
            let out = nums[i - k] as usize;
            let inn = nums[i] as usize;

            {
                let old_f = freq[out];
                if old_f > 0 {
                    set.remove(&(-old_f, -(out as i32)));
                }
                freq[out] -= 1;
                let new_f = freq[out];
                if new_f > 0 {
                    set.insert((-new_f, -(out as i32)));
                }
            }

            {
                let old_f = freq[inn];
                if old_f > 0 {
                    set.remove(&(-old_f, -(inn as i32)));
                }
                freq[inn] += 1;
                let new_f = freq[inn];
                if new_f > 0 {
                    set.insert((-new_f, -(inn as i32)));
                }
            }

            ans.push(Self::current_x_sum(&set, x));
        }

        ans
    }
}