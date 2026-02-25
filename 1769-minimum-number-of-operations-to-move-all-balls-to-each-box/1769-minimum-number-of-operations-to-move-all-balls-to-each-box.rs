impl Solution {
    pub fn min_operations(boxes: String) -> Vec<i32> {
        let boxes = boxes.into_bytes();
        let mut results = vec![0; boxes.len()];

        for left_id in 0 .. boxes.len() {
            if boxes[left_id] == b'0' {
                continue;
            }

            for right_id in 0 .. boxes.len() {
                results[right_id] += (right_id.max(left_id) - right_id.min(left_id)) as i32;
            }
        }

        results
    }
}