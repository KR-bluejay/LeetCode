impl Solution {
    pub fn find_smallest_integer(nums: Vec<i32>, value: i32) -> i32 {
        let mut min_rem = 0;
        let mut num_counts = vec![0; value as usize];

        for &num in nums.iter() {
            num_counts[(num % value).rem_euclid(value) as usize] += 1;
        }

        while num_counts[(min_rem % value) as usize] > 0 {
            num_counts[(min_rem % value) as usize] -= 1;
            min_rem += 1;
        }
        min_rem
    }
}