use std::cmp::max;

impl Solution {
    pub fn number_of_good_paths(vals: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        let n = vals.len();
        // each node is a good path
        let mut ans = n as i32;
        let mut edges = edges;
        // sort by vals
        edges.sort_by(|x, y| max(vals[x[0] as usize], vals[x[1] as usize]).cmp(&max(vals[y[0] as usize], vals[y[1] as usize])));

        // dsu
        let mut cnt = vec![1; n];
        // each element is in its own group initially
        let mut root = (0 .. n).collect();
        fn get(x: usize, root: &mut Vec<usize>) -> usize {
            if x == root[x] {
                return x;
            }
            return get(root[x], root);
        }
        // iterate each edge
        for e in edges {
            // get the root of x
            let x = get(e[0] as usize, &mut root);
            // get the root of y
            let y = get(e[1] as usize, &mut root);
            // if their vals are same, 
            if vals[x] == vals[y] {
                // then there would be cnt[x] * cnt[y] good paths
                ans += cnt[x] as i32 * cnt[y] as i32;
                // unite them
                root[x] = y;
                // add the count of x to that of y
                cnt[y] += cnt[x];
            } else if vals[x] > vals[y] {
                // unite them
                root[y] = x;
            } else {
                // unite them
                root[x] = y;
            }
        }
        ans
    }
}