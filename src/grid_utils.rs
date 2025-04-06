pub fn transpose(grid: &[Vec<char>]) -> Vec<Vec<char>> {
    (0..grid.first().unwrap().len())
        .map(|j| (0..grid.len()).map(move |i| grid[i][j]).collect::<Vec<_>>())
        .collect()
}
