use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn dfs(
        vertex_edge_map: &HashMap<i32, HashSet<i32>>,
        vertex_history_set: &mut HashSet<i32>,
        total_edge_count: &mut usize,
        vertex_id: i32,
    ) {
        vertex_history_set.insert(vertex_id);

        let edge_set: HashSet<i32> = vertex_edge_map.get(&vertex_id).cloned().unwrap_or(HashSet::new());
        let mut rtn = true;

        for edge_id in edge_set.iter() {
            *total_edge_count += 1;
            if vertex_history_set.contains(&edge_id) {
                continue;
            }

            Self::dfs(vertex_edge_map, vertex_history_set, total_edge_count, edge_id.clone());
        }
    }
    pub fn count_complete_components(
        n: i32, 
        edges: Vec<Vec<i32>>
    ) -> i32 {
        let mut vertex_history_set: HashSet<i32> = HashSet::with_capacity(n as usize);
        let mut vertex_edge_map: HashMap<i32, HashSet<i32>> = HashMap::with_capacity(n as usize);

        let mut connected_component_count = 0;

        for list in edges.iter() {
            let start_vertex_id = list[0].clone();
            let end_vertex_id = list[1].clone();

            vertex_edge_map.entry(start_vertex_id).or_insert(HashSet::new()).insert(end_vertex_id);
            vertex_edge_map.entry(end_vertex_id).or_insert(HashSet::new()).insert(start_vertex_id);
        }

        for i in 0 .. n {

            if !vertex_edge_map.contains_key(&i) {
                connected_component_count += 1;
                continue;
            }

            if vertex_history_set.contains(&i) {
                continue;
            }

            let mut total_edge_count: usize = 0;
            let prev_count = vertex_history_set.len().clone();
            
            Self::dfs(&vertex_edge_map, &mut vertex_history_set, &mut total_edge_count, i.clone());

            let after_count = vertex_history_set.len().clone();
            
            let node_count = after_count - prev_count;

            total_edge_count /= 2;
            if node_count == 2 && total_edge_count == 1{
                connected_component_count += 1;
                continue;
            }
            if node_count * (node_count - 1) / 2 == total_edge_count {
                connected_component_count += 1;
            }
        }



        connected_component_count
    }
}