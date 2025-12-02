impl Solution {
    pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
        people.sort();

        let mut boat_count = 0;
        let mut left_id = 0;
        let mut right_id = people.len() - 1;

        while left_id <= right_id && right_id < people.len() {
            boat_count += 1;

            if people[left_id] + people[right_id] <= limit {
                left_id += 1;
                right_id -= 1;
            } else {
                right_id -= 1;
            }
        }



        boat_count
    }
}