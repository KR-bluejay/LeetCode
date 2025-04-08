use std::collections::{ HashMap, HashSet };
impl Solution {
    fn dp(
        id: usize, 
        n: usize, 
        visited: &mut Vec<bool>,
        arrangement_count: &mut i32,
    ) {
        if id == n {
            *arrangement_count += 1;
        }

        for i in 1 ..= n {
            if ((id + 1) % i == 0 || i % (id + 1) == 0) && !visited[i] {
                visited[i] = true;
                Self::dp(id + 1, n, visited, arrangement_count);
                visited[i] = false;
            }
        }
    }
    pub fn count_arrangement(n: i32) -> i32 {
        let n = n as usize;
        let mut visited: Vec<bool> = vec![false; n + 1];
        let mut arr_count = 0;

        Self::dp(0, n, &mut visited, &mut arr_count);

        arr_count
    }
}