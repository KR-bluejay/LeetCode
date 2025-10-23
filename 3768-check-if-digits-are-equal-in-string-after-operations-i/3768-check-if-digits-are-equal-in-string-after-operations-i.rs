impl Solution {
    pub fn has_same_digits(s: String) -> bool {
        let mut nums: Vec<u8> = s.into_bytes().into_iter().map(|v| v - b'0').collect();
        
        while nums.len() > 2 {
            for i in 0 .. nums.len() - 1 {
                nums[i] = (nums[i] + nums[i + 1]) % 10
            }
            nums.pop();
        }

        nums.first().unwrap_or(&0) == nums.last().unwrap_or(&0)
    }
}