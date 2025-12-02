use std::collections::HashMap;

impl Solution {
    fn find_apples(
        id: usize,
        tree_map: &HashMap<usize, Vec<usize>>,
        tree_visit: &mut Vec<bool>,
        has_apple: &Vec<bool>,
    ) -> i32 {
        let mut collecting_time = 0;

        tree_visit[id] = true;

        if let Some(neighbors) = tree_map.get(&id) {
            for &neighbor in neighbors.iter() {
                if tree_visit[neighbor] {
                    continue;
                }

                collecting_time += Self::find_apples(
                    neighbor, 
                    tree_map, 
                    tree_visit,
                    has_apple
                );
            }
        }

        collecting_time + if collecting_time > 0 || has_apple[id] {
            2
        } else {
            0
        }
    }
    pub fn min_time(n: i32, edges: Vec<Vec<i32>>, has_apple: Vec<bool>) -> i32 {
        let tree_count = n as usize;
        let mut tree_map: HashMap<usize, Vec<usize>> = HashMap::with_capacity(tree_count);
        let mut tree_visit: Vec<bool> = vec![false; tree_count];

        for (id, edge) in edges.iter().enumerate() {
            let start_id = edge[0] as usize;
            let end_id = edge[1] as usize;

            tree_map.entry(start_id).or_insert(Vec::with_capacity(3)).push(end_id);
            tree_map.entry(end_id).or_insert(Vec::with_capacity(3)).push(start_id);
        }

        Self::find_apples(0, &tree_map, &mut tree_visit, &has_apple).max(2) - 2
    }
}