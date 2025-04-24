use std::collections::{ HashSet, BTreeSet };
impl Solution {
    fn find_subarray(
        start_id: usize, 
        cur_id: usize,
        nums: &Vec<i32>, 
        subset: &mut BTreeSet<i32>,
        num_dist_count: usize,
        visited: &mut Vec<Vec<bool>>
    ) -> usize {
        if cur_id == nums.len() || visited[start_id][cur_id] {
            return 0;
        }

        let num_item = nums[cur_id];
        // Skip
        let skip_count = Self::find_subarray(cur_id + 1, cur_id + 1, nums, &mut BTreeSet::new(), num_dist_count, visited);
        
        visited[start_id][cur_id] = true;
        subset.insert(nums[cur_id]);
        let contain_count: usize = if subset.len() == num_dist_count {
            visited[start_id][cur_id .. nums.len()].fill(true);
            nums.len() - cur_id
        } else {
            Self::find_subarray(start_id, cur_id + 1, nums, subset, num_dist_count, visited)
        };


        skip_count + contain_count
    }
    pub fn count_complete_subarrays(nums: Vec<i32>) -> i32 {
        let num_set: HashSet<_> = nums.iter().collect();
        let num_dist_count = num_set.len();
        let mut visited: Vec<Vec<bool>> = vec![vec![false; nums.len()]; nums.len()];
        let mut subset: BTreeSet<i32> = BTreeSet::new();


        Self::find_subarray(0, 0, &nums, &mut subset, num_dist_count, &mut visited) as i32
    }
}