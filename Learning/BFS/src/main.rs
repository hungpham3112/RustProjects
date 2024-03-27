use std::collections::VecDeque;

// Define a structure to represent a maze
struct Maze {
    rows: usize,
    cols: usize,
    start_row: usize,
    start_col: usize,
    grid: Vec<Vec<u8>>,
}

impl Maze {
    // Constructor to create a new maze
    fn new(rows: usize, cols: usize, start_row: usize, start_col: usize, grid: Vec<Vec<u8>>) -> Self {
        Maze { rows, cols, start_row, start_col, grid }
    }

    // Perform breadth-first search to find the shortest path to exit the maze
    fn bfs(&self) -> i32 {
        let mut visited = vec![vec![false; self.cols]; self.rows];
        let mut queue = VecDeque::new();

        // Enqueue the starting position and mark it as visited
        queue.push_back((self.start_row, self.start_col, 0));
        visited[self.start_row][self.start_col] = true;

        // Define the four possible directions: up, down, left, right
        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        // Perform BFS
        while let Some((x, y, steps)) = queue.pop_front() {
            if self.grid[x][y] == 1 {
                continue; // Skip if the cell is a wall
            }
            if x == 0 || x == self.rows - 1 || y == 0 || y == self.cols - 1 {
                return steps; // Found the exit
            }
            // Explore the four directions
            for &(dx, dy) in &directions {
                let new_x = (x as i32 + dx) as usize;
                let new_y = (y as i32 + dy) as usize;
                if new_x < self.rows && new_y < self.cols && !visited[new_x][new_y] && self.grid[new_x][new_y] == 0 {
                    visited[new_x][new_y] = true;
                    queue.push_back((new_x, new_y, steps + 1));
                }
            }
        }

        // If no path to exit is found
        -1
    }
}

fn main() {
    // Read input
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    let r: usize = iter.next().unwrap().parse().unwrap();
    let c: usize = iter.next().unwrap().parse().unwrap();

    let mut grid = vec![];
    for _ in 0..n {
        let mut row = String::new();
        std::io::stdin().read_line(&mut row).unwrap();
        let row: Vec<u8> = row.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
        grid.push(row);
    }

    // Create a maze
    let maze = Maze::new(n, m, r - 1, c - 1, grid);

    // Perform BFS to find the shortest path to exit
    let shortest_path = maze.bfs();

    // Print the result
    println!("{}", shortest_path + 1);
}
