pub fn transpose(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    return (0..grid.first().unwrap().len())
        .into_iter()
        .map(|j| {
            (0..grid.len())
                .into_iter()
                .map(move |i| grid[i][j])
                .collect::<Vec<char>>()
        })
        .collect();
}
