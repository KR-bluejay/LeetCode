impl Solution {
    fn dfs(edge_id: usize, graph: &Vec<Vec<usize>>, visited: &mut Vec<bool>) {
        visited[edge_id] = true;

        for &neighbor_id in graph[edge_id].iter() {
            if visited[neighbor_id] {
                continue;
            }

            Self::dfs(neighbor_id, graph, visited);
        }
    }
    pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut visited: Vec<bool> = vec![false; n];
        let mut graph: Vec<Vec<usize>> = vec![Vec::with_capacity(n); n];
        let mut connected_count: i32 = 0;

        for i in 0 .. edges.len() {
            let start_id = edges[i][0] as usize;
            let dest_id = edges[i][1] as usize;

            graph[start_id].push(dest_id);
            graph[dest_id].push(start_id);
        }

        for i in 0 .. n {
            if visited[i] {
                continue;
            }
            Self::dfs(i, &graph, &mut visited);
            connected_count += 1;
        }
        connected_count
    }
}