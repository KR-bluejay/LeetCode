impl Solution {
    pub fn can_make_arithmetic_progression(mut arr: Vec<i32>) -> bool {
        arr.sort_unstable();
        let mut diff = arr[1] - arr[0];

        for id in 1 .. arr.len() - 1{
            if diff != arr[id + 1] - arr[id] {
                return false;
            }
        }

        true
    }
}