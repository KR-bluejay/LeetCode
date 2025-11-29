impl Solution {
    pub fn max_k_divisible_components(
        n: i32, 
        edges: Vec<Vec<i32>>, 
        values: Vec<i32>, 
        k: i32
    ) -> i32 {
        let n = n as usize;
        let mut graph = vec![vec![]; n];
        let mut visited = vec![false; n];
        
        // Build adjacency list
        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            graph[u].push(v);
            graph[v].push(u);
        }
        
        let mut components = 0;
        
        fn dfs(
            node: usize,
            parent: usize,
            graph: &Vec<Vec<usize>>,
            values: &Vec<i32>,
            k: i32,
            components: &mut i32,
        ) -> i64 {
            let mut sum = values[node] as i64;
            
            for &neighbor in &graph[node] {
                if neighbor == parent {
                    continue;
                }
                let child_sum = dfs(neighbor, node, graph, values, k, components);
                sum += child_sum;
            }
            
            // If the sum of this subtree is divisible by k, we can split it off
            if sum % (k as i64) == 0 {
                *components += 1;
                return 0; // This component is complete, return 0 to parent
            }
            
            sum
        }
        
        dfs(0, usize::MAX, &graph, &values, k, &mut components);
        components
    }
}