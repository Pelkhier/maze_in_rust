#![allow(unused)]

mod maze;
use maze::{Cell, Maze};

fn main() {
    let rows = 10;
    let cols = 20;
    let start_row = 8;
    let start_col = 4;

    let target_row = rows - 3;
    let target_col = cols - 1;

    let mut maze = Maze::new(rows, cols);
    maze.generate_maze(start_row, start_col, target_row, target_col);
    maze.print();
    println!("---------------------------------------------------");
    maze.solve_maze(start_row, start_col, target_row, target_col);
    maze.print();
}
