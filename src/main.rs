use std::{thread, time};
// Width and height of the cells array
const NUM_COLS: usize = 31;
const NUM_ROWS: usize = 11;
// Choose types that can fit the NUM_COLS and the NUM_ROWS accordingly.
type CellsType = [[bool; NUM_COLS]; NUM_ROWS]; // So access items with arr[row_i][col_i].

fn draw(cells: &CellsType, step_num: u8) {
    println!("Step: {}", step_num);
    for row in cells.iter() {
        for cell in row.iter() {
            print!("{}", if *cell { 'x' } else { '_' });
        }
        print!("\n");
    }
} 

fn main() {
    let mut cells: CellsType = [[false; NUM_COLS]; NUM_ROWS];
    let mut cells_next: CellsType = cells; // TODO can we not initialize this?
    let mut step_num = 1;
    // Glider
    cells[0][1] = true;
    cells[1][2] = true;
    cells[2][0] = true;
    cells[2][1] = true;
    cells[2][2] = true;
    loop {
        draw(&cells, step_num);
        thread::sleep(time::Duration::from_millis(500));
        step_num += 1;
    }
}
