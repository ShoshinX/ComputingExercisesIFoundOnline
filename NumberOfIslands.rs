impl Solution {
    fn dfs(vis: &mut Vec<i32>, grid: &Vec<Vec<char>>, mark: i32, m:usize, n:usize, i:usize, j:usize){
        if i >= m || j >= n || i < 0 || j < 0 || vis[i *n + j] != 0 || grid[i][j] == '0' {
            return
        }
        vis[i*n + j] = mark;
        Solution::dfs(vis, grid, mark, m, n, i - 1, j);
        Solution::dfs(vis, grid, mark, m, n, i + 1, j);
        Solution::dfs(vis, grid, mark, m, n, i, j - 1);
        Solution::dfs(vis, grid, mark, m, n, i, j + 1);
    }
    
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let (m,n) = (grid.len(),grid[0].len());
        let mut vis = vec![0;m*n];
        let mut cnt = 0;
        for i in 0..m{
            for j in 0..n{
                if grid[i][j] == '1' && vis[i*n+j] == 0{
                    cnt += 1;
                    Solution::dfs(&mut vis, &grid, cnt, m,n,i,j)
                }
            }
        }
        cnt
    }
}
