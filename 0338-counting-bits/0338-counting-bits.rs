impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut ans: Vec<i32> = Vec::with_capacity(1 + n);

        for i in 0 ..= n {
            let cnt = format!("{:b}", i).chars().filter(|c| *c == '1').count();

            ans.push(cnt as i32);
        }
        ans
    }
}