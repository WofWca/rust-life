use life::{step_toroidal, parse_life, Cells, CellState, draw, init_board, TableLike};

use std::{thread, time};

fn main() {
    let mut step_num: u32 = 1;
    let filename = std::env::args().nth(1).unwrap();
    println!("File: {}", filename);
    let file_content = std::fs::read_to_string(filename).unwrap();
    println!("Content: {}", file_content);
    let mut cells = parse_life(file_content).unwrap();
    let mut cells_next: Cells = vec![vec![CellState::Dead; cells.num_cols()]; cells.num_rows()]; // TODO can we not initialize this?

    init_board(cells.num_rows(), cells.num_cols());
    loop {
        draw(&cells, &step_num);
        thread::sleep(time::Duration::from_millis(30));
        step_toroidal(&cells, &mut cells_next);
        std::mem::swap(&mut cells, &mut cells_next);
        step_num += 1;
    }
}
