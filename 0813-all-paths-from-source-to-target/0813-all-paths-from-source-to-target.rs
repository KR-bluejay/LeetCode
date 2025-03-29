impl Solution {
    fn dfs(cur: &mut Vec<i32>, graph: &Vec<Vec<i32>>, visited: &mut Vec<bool>, all_path: &mut Vec<Vec<i32>>, i: usize) {
        visited[i] = true;
        cur.push(i as i32);

        if i == graph.len() - 1 {
            all_path.push(cur.to_vec());

            return;
        }

        for graph_item in graph[i].iter() {
            if visited[*graph_item as usize] {
                continue
            }

            Self::dfs(cur, graph, visited, all_path, *graph_item as usize);
            
            visited[*graph_item as usize] = false;
            cur.pop();
        }
    }
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut visited: Vec<bool> = vec![false; graph.len()];
        let mut all_path: Vec<Vec<i32>> = Vec::new();
        let mut cur: Vec<i32> = Vec::with_capacity(graph.len());
        
        Self::dfs(&mut cur, &graph, &mut visited, &mut all_path, 0);

        all_path
    }
}