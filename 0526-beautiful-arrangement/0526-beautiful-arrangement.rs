use std::collections::{ HashMap, HashSet };
impl Solution {
    fn dp(
        id: usize, 
        n: usize, 
        location_map: &HashMap<usize, HashSet<usize>>, 
        history: &mut Vec<usize>, 
    ) -> usize {
        let mut res = 0;


        let avail_index_set = match location_map.get(&id) {
            Some(v) => v,
            None => {
                println!("{id}");
                println!("{history:?}");
                return 1;
            },
        };

        for &avail_index in avail_index_set.iter() {
            if history.contains(&avail_index) {
                continue;
            }

            history.push(avail_index);
            res += Self::dp(id + 1, n, location_map, history);
            history.pop();
        }

        return res;
    }
    pub fn count_arrangement(n: i32) -> i32 {
        let n = n as usize;
        let mut location_map: HashMap<usize, HashSet<usize>> = HashMap::new();
        let mut history: Vec<usize> = Vec::with_capacity(n);

        for i in 1 ..= n {
            for j in 1 ..= n {
                if i % j == 0 || j % i == 0 {
                    location_map.entry(j - 1).or_insert(HashSet::new()).insert(i);
                }
            }
        }

        Self::dp(0, n, &location_map, &mut history) as i32
    }
}