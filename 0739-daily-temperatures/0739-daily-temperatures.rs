impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut results = vec![0; temperatures.len()];
        let mut temp_stack = Vec::with_capacity(temperatures.len());

        for right_id in 0 .. temperatures.len() {
            while let Some(&left_id) = temp_stack.last() 
            && temperatures[left_id] < temperatures[right_id] {
                results[left_id] = (right_id - left_id) as i32;
                
                temp_stack.pop();
            }

            temp_stack.push(right_id);
        }

        results
    }
}