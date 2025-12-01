impl Solution {
    pub fn search(mut nums: Vec<i32>, target: i32) -> bool {
       nums.sort_unstable();

       nums.binary_search(&target).is_ok()
    }
}