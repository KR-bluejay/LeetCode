use std::cmp::{ Reverse, Ordering };

#[derive(Copy, Clone, Debug)]
struct Edge {
    u: usize,
    v: usize,
    w: i32,
}

#[derive(Clone, Debug)]
struct DSU {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self {
            parent: (0 .. n).collect(),
            rank: vec![0; n],
        }
    }

    fn find(&mut self, id: usize) -> usize {
        if self.parent[id] != id {
            self.parent[id] = self.find(self.parent[id]);
        }

        self.parent[id]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let px = self.find(x);
        let py = self.find(y);

        if px == py {
            return false;
        }

        match self.rank[px].cmp(&self.rank[py]) {
            Ordering::Less => {
                self.parent[px] = py;
            },
            Ordering::Equal => {
                self.parent[py] = px;
            },
            Ordering::Greater => {
                self.parent[px] = py;
                self.rank[px] += 1;
            },
        }

        true
    }
}
impl Solution {
    pub fn max_stability(n: i32, _edges: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = n as usize;
        
        let mut initial_dsu = DSU::new(n);
        let mut edges: Vec<Vec<Edge>> = vec![vec![]; 2];

        let (mut low, mut high) = (0, i32::MAX);
        let mut result = -1;

        for e in _edges.iter() {
            let edge = Edge{
                u: e[0] as usize,
                v: e[1] as usize,
                w: e[2],
            };

            if e[3] == 1 {
                if !initial_dsu.union(edge.u, edge.v) {
                    return -1;
                }

                high = high.min(e[2]);
            }

            edges[e[3] as usize].push(edge);
        }

        edges[0].sort_unstable_by_key(|e| Reverse(e.w));


        while low <= high {
            let mid = (low + high) / 2;
            let mut dsu = initial_dsu.clone();

            let (mut cur_k, mut must_count) = (k, edges[1].len());

            for &edge in edges[0].iter() {
                let Edge { u, v, w } = edge;

                if must_count + 1 == n {
                    break;
                }

                if ((w >= mid) || (cur_k > 0 && w * 2 >= mid)) 
                && dsu.union(u, v) {
                    if w < mid {
                        cur_k -= 1;
                    }
                    must_count += 1;
                }
            }

            if must_count + 1 == n {
                result = mid;
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }


        result
    }
}