// COPYRIGHT (C) 2022 barreiro. All Rights Reserved.
// Rust solvers for Project Euler problems

use algorithm::cast::Cast;
use algorithm::io::load_default_data;
use Solver;

/// The following undirected network consists of seven vertices and twelve edges with a total weight of `243`.
///
/// [ see image ]
///
/// The same network can be represented by the matrix below.
/// ```
///      A    B    C    D    E    F    G
/// A    -   16   12   21    -    -    -
/// B   16    -    -   17   20    -    -
/// C   12    -    -   28    -   31    -
/// D   21   17   28    -   18   19   23
/// E    -   20    -   18    -    -   11
/// F    -    -   31   19    -    -   27
/// G    -    -    -   23   11   27    -
/// ```
/// However, it is possible to optimise the network by removing some edges and still ensure that all points on the network remain connected.
/// The network which achieves the maximum saving is shown below. It has a weight of `93`, representing a saving of `243 − 93 = 150` from the original network.
///
/// [ see image ]
///
/// Using network.txt (right click and 'Save Link/Target As...'), a 6K text file containing a network with forty vertices, and given in matrix form, find the maximum saving which can be achieved by removing redundant edges whilst ensuring that the network remains connected.
pub struct Solver107 {
    pub n: usize,
    pub input: String,
}

impl Default for Solver107 {
    fn default() -> Self {
        Self { n: 40, input: load_default_data(107) }
    }
}

impl Solver for Solver107 {
    fn solve(&self) -> i64 {
        let matrix = self.input.lines().take(self.n).map(|l| l.split(',').take(self.n).map(|d| d.parse::<u64>().ok()).collect::<Vec<_>>()).collect::<Vec<_>>();
        let mut edges = (0..matrix.len()).flat_map(|i| (0..i).filter_map(|j| matrix[i][j].map(|w| (w, i, j))).collect::<Vec<_>>()).collect::<Vec<_>>();
        edges.sort_unstable();

        // // verify if is still possible to go from start to end after removing the edge from start to end
        // let is_connected = |matrix: &Vec<Vec<Option<_>>>, start: usize, end: usize| {
        //     let (mut visited, mut neighbours) = (vec![false; matrix.len()], vec![start]);
        //     while !neighbours.is_empty() {
        //         neighbours.pop().filter(|&n| !visited[n]).iter().for_each(|&n| {
        //             if matrix[n][end].is_some() {
        //                 visited[end] = true;
        //                 neighbours.clear();
        //             } else {
        //                 visited[n] = true;
        //                 (0..matrix.len()).filter(|&other| !visited[other] && matrix[n][other].is_some()).for_each(|other| neighbours.push(other));
        //             }
        //         });
        //     }
        //     visited[end]
        // };
        //
        // // implement reverse delete algorithm (reverse Kruskal) by removing the biggest edges while maintaining the graph connected
        // // the sum of the edges removed is the saving
        // edges.iter().rev().filter_map(|&(w, from, to)| {
        //     let previous = matrix[from][to];
        //     (matrix[from][to], matrix[to][from]) = (None, None);
        //     if is_connected(&matrix, from, to) {
        //         Some(w).map(to_i64)
        //     } else {
        //         (matrix[from][to], matrix[to][from]) = (previous, previous);
        //         None
        //     }
        // }).sum()

        // implemented with Kruskal's algorithm. add the smallest edges that do not introduce cycles. removed edges are savings
        // let (mut visited, mut group) = (, 0);
        // edges.iter().filter_map(|&(w, from, to)| {
        //     if visited[from].and(visited[to]).is_none() {
        //         let current_group = visited[from].or(visited[to]).or_else(|| Some(group.increment_and_get()));
        //         (visited[from], visited[to]) = (current_group, current_group);
        //         None
        //     } else if visited[from] != visited[to] {
        //         let (min, max) = (visited[from].min(visited[to]), visited[from].max(visited[to]));
        //         visited.iter_mut().filter(|v| **v == max).for_each(|v| *v = min);
        //         None
        //     } else {
        //         Some(w)
        //     }
        // }).sum()

        // implemented with Prim's algorithm. start with one vertex and grow the minimum tree from there, adding one unvisited vertx at a time
        let mut visited = vec![false; matrix.len()];
        visited[matrix.len() - 1] = true;
        while visited.iter().any(|&v| !v) {
            let (w, from, to) = edges.iter_mut().find(|&&mut (w, from, to)| w != 0 && visited[from] ^ visited[to]).expect("Graph should be connected");
            (visited[*from], visited[*to]) = (true, true);
            *w = 0; // reset this edge. in the end, the remaining edges are the ones that do not form the MST
        }
        edges.iter().map(|(w, _, _)| w.as_i64()).sum()
    }
}
