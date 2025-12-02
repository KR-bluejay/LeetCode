impl Solution {
    fn find_min_time(
        id: usize, 
        parent_id: usize, 
        graph: &Vec<Vec<usize>>,
        has_apple: &Vec<bool>,
    ) -> i32 {
        let mut collecting_time = 0;

        for &child_id in graph[id].iter() {
            if parent_id == child_id {
                continue;
            }
            
            let child_time = Self::find_min_time(child_id, id, graph, has_apple);
            
            if child_time > 0 || has_apple[child_id] {
                collecting_time += child_time + 2;
            }
        }

        collecting_time
    }
    pub fn min_time(n: i32, edges: Vec<Vec<i32>>, has_apple: Vec<bool>) -> i32 {
        let tree_count = n as usize;
        let mut graph: Vec<Vec<usize>> = vec![Vec::with_capacity(3); tree_count];

        for edge in edges {
            let (from, to) = (edge[0] as usize, edge[1] as usize);

            graph[from].push(to);
            graph[to].push(from);
        }

        Self::find_min_time(0, usize::MAX, &graph, &has_apple)
    }
}