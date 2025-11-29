impl Solution {
    fn count_components(
        node_id: usize,
        parent_id: Option<usize>,
        values: &Vec<i32>,
        graph: &Vec<Vec<usize>>,
        neighbor_nodes: &Vec<usize>,
        k: i32,
    ) -> (i32, i32) {
        let mut comp_count = 0;
        let mut comp_sum = 0;

        for &neighbor_node in  neighbor_nodes.iter() {
            if parent_id.is_some() && parent_id.unwrap() == neighbor_node {
                continue;
            }

            let (local_count, local_sum) = Self::count_components(
                neighbor_node,
                Some(node_id),
                values,
                graph,
                &graph[neighbor_node],
                k
            );
            comp_count += local_count;

            comp_sum += local_sum;
            comp_sum %= k;
        }
        comp_sum += values[node_id];
        comp_sum %= k;

        if comp_sum == 0 {
            comp_count += 1;
        }

        (comp_count, comp_sum)
    }
    pub fn max_k_divisible_components(
        n: i32, 
        edges: Vec<Vec<i32>>, 
        values: Vec<i32>, 
        k: i32
    ) -> i32 {
        let node_count = n as usize;
        let mut graph: Vec<Vec<usize>> = vec![Vec::with_capacity(node_count); node_count];

        for edge in edges {
            let (node1, node2) = (edge[0] as usize, edge[1] as usize);

            graph[node1].push(node2);
            graph[node2].push(node1);
        }

        Self::count_components(0, None, &values, &graph, &graph[0], k).0
    }
}